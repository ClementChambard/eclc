use super::symbol::Symbol;

pub struct RuleStrings {
    pub left: String,
    pub right: Vec<String>,
    pub prio: crate::grammar_file::RulePriorities,
    pub ast: crate::parser::ast::AstDef,
}

impl RuleStrings {
    pub fn string_repr(&self) -> String {
        let mut s = String::new();
        s.push_str(&format!("{} ::= ", self.left));
        for r in &self.right {
            s.push_str(r);
            s.push(' ');
        }
        s
    }
}

#[derive(Debug, Clone)]
pub struct Rule {
    pub left: String,
    pub right: Vec<Symbol>,
}

impl std::fmt::Display for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ::= ", self.left)?;
        for i in self.right.iter() {
            write!(f, "{} ", i)?;
        }
        Ok(())
    }
}

impl std::fmt::Display for RuleStrings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ::= ", self.left)?;
        for i in self.right.iter() {
            write!(f, "{} ", i)?;
        }
        Ok(())
    }
}
