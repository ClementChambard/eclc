use super::*;

pub fn is_ll1_grammar(grammar: &Grammar) -> bool {
    let mut ll1 = true;

    for non_terminal in &grammar.non_term {
        let mut first_sets_for_rules: HashMap<Symbol, Rule> = HashMap::new();

        for rule in &grammar.rules {
            if rule.left == *non_terminal {
                let mut first_set = grammar.getfirst(&rule.right);
                if let Some(ref mut first_set) = first_set {
                    for symbol in first_set.iter() {
                        if first_sets_for_rules.contains_key(symbol) {
                            // Conflict detected, print the conflicting rules
                            ll1 = false;
                            println!("Conflict for non-terminal {:?}:", non_terminal);
                            println!("Rule 1: {:?} -> {:?}", rule.left, rule.right);
                            let rule2 = &first_sets_for_rules[symbol];
                            println!("Rule 2: {:?} -> {:?}", rule2.left, rule2.right);
                        } else {
                            first_sets_for_rules.insert(symbol.clone(), rule.clone());
                        }
                    }
                }
                if grammar.can_derive_epsilon(&rule.right) {
                    // ε (epsilon) is in the FIRST set of α, check FOLLOW and FIRST sets
                    let follow_set_for_rule = &grammar.follow_sets[&rule.left];
                    for symbol in follow_set_for_rule {
                        if first_sets_for_rules.contains_key(symbol) {
                            // Conflict detected, print the conflicting rules
                            ll1 = false;
                            println!("Conflict for non-terminal {:?}:", non_terminal);
                            println!("Rule 1: {:?} -> {:?}", rule.left, rule.right);
                            let rule2 = &first_sets_for_rules[symbol];
                            println!("Rule 2: {:?} -> {:?}", rule2.left, rule2.right);
                        }
                    }
                }
            }
        }
    }

    ll1
}
