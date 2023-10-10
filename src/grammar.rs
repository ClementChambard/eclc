mod ll1_check;
mod production_table;
pub mod rules;
mod symbol;

pub use ll1_check::is_ll1_grammar;
pub use production_table::{get_production_table_entry, ProductionTable, ProductionTableEntry};
pub use rules::Rule;
pub use symbol::Symbol;
pub use symbol::SymbolName;

use std::collections::{HashMap, HashSet};
use std::io::Write;

#[derive(Debug)]
pub struct Grammar {
    rules: Vec<Rule>,
    non_term: Vec<String>,
    first_sets: HashMap<String, HashSet<Symbol>>,
    follow_sets: HashMap<String, HashSet<Symbol>>,
    ast_prod: HashMap<String, crate::parser::ast::AstDef>,
}

impl Grammar {
    pub fn _gen_rust(&self) -> Result<(), std::io::Error> {
        for (k, v) in &self.ast_prod {
            println!("ast_prod.insert(\"{}\", {:?});", k, v);
        }
        let production_table = self.fill_ll1_production_table();
        let f = std::fs::File::create("test.rs")?;
        let mut buf = std::io::BufWriter::new(f);
        production_table::_gen_rust_for_production_table(&mut buf, &production_table)?;
        buf.flush()?;
        Ok(())
    }

    pub fn get_ast_prod(&self) -> HashMap<String, crate::parser::ast::AstDef> {
        self.ast_prod.clone()
    }

    pub fn from_rule_string(rulestrings: Vec<rules::RuleStrings>) -> Self {
        let mut rls = vec![];
        let mut nts = std::collections::HashSet::new();
        let mut prods = HashMap::new();
        for r in &rulestrings {
            nts.insert(r.left.clone());
            prods.insert(r.string_repr(), r.ast.clone());
        }
        for r in rulestrings {
            let mut rule = Rule {
                left: r.left,
                right: vec![],
            };
            for s in r.right {
                if nts.contains(&s) {
                    rule.right.push(Symbol::NT(s));
                } else {
                    rule.right.push(Symbol::T(s));
                }
            }
            rls.push(rule);
        }

        Self {
            rules: rls,
            non_term: nts.into_iter().collect(),
            first_sets: HashMap::new(),
            follow_sets: HashMap::new(),
            ast_prod: prods,
        }
    }

    pub fn calculate_first_sets(&mut self) {
        for nt in &self.non_term {
            self.first_sets.insert(nt.clone(), HashSet::new());
        }

        let mut changed = true;
        while changed {
            changed = false;
            for rule in &self.rules {
                let left = &rule.left;
                let right = &rule.right;
                let mut add_eps = true;
                for symbol in right {
                    if let Symbol::T(term) = symbol {
                        if !self.first_sets[left].contains(&Symbol::T(term.clone())) {
                            self.first_sets
                                .get_mut(left)
                                .unwrap()
                                .insert(Symbol::T(term.clone()));
                            changed = true;
                        }
                        add_eps = false;
                        break;
                    } else if let Symbol::NT(nt) = symbol {
                        let first_nt = self.first_sets[nt].clone();
                        for s in &first_nt {
                            if !self.first_sets[left].contains(s) {
                                self.first_sets.get_mut(left).unwrap().insert(s.clone());
                                changed = true;
                            }
                        }
                        if !first_nt.contains(&Symbol::T("epsilon".to_string())) {
                            add_eps = false;
                            break;
                        }
                    }
                }
                if add_eps && !self.first_sets[left].contains(&Symbol::T("epsilon".to_string())) {
                    self.first_sets
                        .get_mut(left)
                        .unwrap()
                        .insert(Symbol::T("epsilon".to_string()));
                    changed = true;
                }
            }
        }
    }

    pub fn calculate_follow_sets(&mut self) {
        // Initialize follow sets with empty sets
        for symbol in &self.non_term {
            self.follow_sets.insert(symbol.clone(), HashSet::new());
        }

        let mut changed = true;
        // repeat until the sets are stabilized
        while changed {
            changed = false;

            // check for each rule
            for rule in &self.rules {
                // each symbol
                for (i, symbol) in rule.right.iter().enumerate() {
                    // if it is a non terminal.
                    if let Symbol::NT(nt) = symbol {
                        // in which case, this the symbols after it.
                        let mut rest = Vec::new();
                        for j in (i + 1)..rule.right.len() {
                            rest.push(rule.right[j].clone());
                        }

                        if let Some(first) = self.getfirst(&rest) {
                            let follow_set = self.follow_sets.get_mut(nt).unwrap();
                            let old_len = follow_set.len();
                            follow_set.extend(first);
                            if follow_set.len() > old_len {
                                changed = true;
                            }
                        }

                        if self.can_derive_epsilon(&rest) {
                            let follow_set_a = self.follow_sets[&rule.left].clone();
                            let follow_set_b = self.follow_sets.get_mut(nt).unwrap();
                            let old_len = follow_set_b.len();
                            follow_set_b.extend(follow_set_a.iter().cloned());
                            if follow_set_b.len() > old_len {
                                changed = true;
                            }
                        }
                    }
                }
            }
        }
    }

    fn getfirst(&self, symbols: &[Symbol]) -> Option<HashSet<Symbol>> {
        let mut out = HashSet::new();
        for s in symbols {
            match s {
                Symbol::T(_) => {
                    out.insert(s.clone());
                    break;
                }
                Symbol::NT(s) => {
                    let mut fsts = self.first_sets[s].clone();
                    if fsts.remove(&Symbol::T("epsilon".to_string())) {
                        out.extend(fsts.clone());
                        continue;
                    }
                    out.extend(fsts.clone());
                    break;
                }
            }
        }
        if out.is_empty() {
            None
        } else {
            Some(out)
        }
    }

    fn can_derive_epsilon(&self, symbols: &[Symbol]) -> bool {
        for s in symbols {
            match s {
                Symbol::T(s) => {
                    if s == "epsilon" {
                        continue;
                    }
                    return false;
                }
                Symbol::NT(s) => {
                    let fsts = &self.first_sets[s];
                    if !fsts.contains(&Symbol::T("epsilon".to_string())) {
                        return false;
                    }
                }
            }
        }
        true
    }

    pub fn fill_ll1_production_table(&self) -> ProductionTable {
        let mut production_table: ProductionTable = Vec::new();

        // Iterate through each rule in the grammar.
        for rule in &self.rules {
            let non_terminal = &rule.left;

            // Calculate the FIRST set for the right-hand side of the rule.
            let mut first_set = self.getfirst(&rule.right).unwrap_or(HashSet::new());

            // If the rule is nullable, add the FOLLOW set for the non-terminal.
            if self.can_derive_epsilon(&rule.right) {
                let follow_set = &self.follow_sets[non_terminal];
                first_set.extend(follow_set.iter().cloned());
            }

            // For each terminal in the FIRST set, create a production table entry.
            for terminal in first_set {
                if terminal.symbol_name() != "epsilon" {
                    let mut r = rule.right.clone();
                    r.retain(|x| x.symbol_name() != "epsilon");
                    production_table.push(ProductionTableEntry::new(
                        non_terminal,
                        terminal.symbol_name(),
                        r,
                    ));
                }
            }
        }

        production_table
    }
}

impl std::fmt::Display for Grammar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for r in &self.rules {
            writeln!(f, "{}", r)?;
        }
        Ok(())
    }
}
