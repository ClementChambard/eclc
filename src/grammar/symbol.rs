#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Symbol {
    T(String),
    NT(String),
}

// Helper trait to get the name of a Symbol
pub trait SymbolName {
    fn symbol_name(&self) -> &str;
}

impl SymbolName for Symbol {
    fn symbol_name(&self) -> &str {
        match self {
            Symbol::T(name) | Symbol::NT(name) => name,
        }
    }
}

use std::io::BufWriter;
use std::io::Write;

impl Symbol {
    pub fn _gen_rust<W: Write>(&self, buf: &mut BufWriter<W>) -> Result<(), std::io::Error> {
        match self {
            Symbol::T(name) => write!(buf, "Symbol::T(\"{}\".to_owned())", name),
            Symbol::NT(name) => write!(buf, "Symbol::NT(\"{}\".to_owned())", name),
        }
    }
}

impl std::fmt::Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::T(s) => write!(f, "Token(\"{}\")", s),
            Self::NT(s) => write!(f, "{}", s),
        }
    }
}
