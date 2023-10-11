mod micro_lang;

use crate::error::Error;
use crate::lexer::Token;
pub use micro_lang::{parse_ast_def, AstDef};

#[derive(Debug)]
pub enum Node<'a, 'b> {
    NT(String, Vec<Node<'a, 'b>>),
    T(Token<'a, &'b str>),
}

impl<'a, 'b> Node<'a, 'b> {
    pub fn name(&self) -> &str {
        match self {
            Self::T(Token {
                kind,
                loc: _,
                text: _,
            }) => &kind,
            Self::NT(s, _) => &s,
        }
    }
    pub fn string_rep(&self) -> String {
        match self {
            Self::T(Token {
                kind,
                loc: _,
                text: _,
            }) => kind.to_string(),
            Self::NT(s, c) => {
                let mut s = s.clone();
                s.push_str(" ::= ");
                if c.is_empty() {
                    s.push_str("epsilon ");
                }
                for ch in c {
                    s.push_str(ch.name());
                    s.push(' ');
                }
                s
            }
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct AstNode {
    pub name: String, // or content
    pub children: Vec<AstNode>,
} // separate T, NT ?

pub trait NeededForAstNode: Clone {
    fn from_token(tok: &Token<&str>) -> Result<Self, Error>;
}

impl NeededForAstNode for AstNode {
    fn from_token(tok: &Token<&str>) -> Result<Self, Error> {
        Ok(Self {
            name: format!("{}({})", tok.kind, tok.text),
            children: vec![],
        })
    }
}

use std::collections::HashMap;

#[derive(Default)]
pub struct FnExecutor<N: NeededForAstNode> {
    functions: HashMap<String, Box<dyn Fn(&Vec<String>, &Vec<N>) -> Result<N, Error>>>,
}

impl<N: NeededForAstNode> FnExecutor<N> {
    pub fn exec(&self, name: &Vec<String>, params: &Vec<N>) -> Result<N, Error> {
        assert!(name.len() > 0);
        let fn_name = &name[0];
        let name_add = &name[1..];
        let f = self.functions.get(fn_name).unwrap();
        f(&name_add.to_vec(), params)
    }
}

#[derive(Default)]
pub struct AstResolver<N: NeededForAstNode> {
    executor: FnExecutor<N>,
    map: HashMap<String, AstDef>,
}

impl<N: NeededForAstNode> AstResolver<N> {
    pub fn set_ast_prod(&mut self, map: HashMap<String, AstDef>) {
        self.map = map;
    }
    pub fn resolve(&self, node: &Node, params: &Vec<N>) -> Result<N, Error> {
        match node {
            Node::NT(_, c) => {
                let str_rep = node.string_rep();
                let prod = self.map.get(&str_rep).unwrap();
                prod.execute(&self, &c, params)
            }
            Node::T(tok) => N::from_token(tok),
        }
    }

    pub fn add_func<T>(&mut self, name: &str, f: T)
    where
        T: Fn(&Vec<String>, &Vec<N>) -> Result<N, Error> + 'static,
    {
        self.executor
            .functions
            .insert(name.to_string(), Box::new(f));
    }
}
