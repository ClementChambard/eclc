use super::*;

use crate::parser::ast::NeededForAstNode;
use magic_unwrapper::EnumUnwrap;

#[derive(Debug, Clone, Default, EnumUnwrap)]
pub enum AstNode {
    Ecl(Ecl),
    Sub(Sub),
    Param(Param),
    Instr(Instr),
    Expr(Expr),

    Token(Token),

    Data {
        dtype: String,
        children: Vec<AstNode>,
    },
    List(Vec<AstNode>),
    #[default]
    None,
}

impl AstNode {
    pub fn data(self) -> (String, Vec<AstNode>) {
        let Self::Data{dtype, children} = self else { panic!(); };
        (dtype, children)
    }
    pub fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }
}

impl NeededForAstNode for AstNode {
    fn from_token(tok: &crate::lexer::Token<&str>) -> Result<Self, Error> {
        Ok(Self::Token(Token::from(tok)))
    }
}
