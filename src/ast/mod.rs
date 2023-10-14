mod builtin_idents;
mod ecl;
mod expr;
mod if_construct;
mod instr;
mod located;
mod loop_construct;
mod node;
mod special_ast_nodes;
mod sub;
mod tok_name_for_error;
mod token;
mod tokens_to_vals;
mod variables;
mod while_construct;
pub use ecl::Ecl;
pub use expr::{Expr, ExprType};
pub use instr::{Instr, TimeLabelKind};
pub use located::Located;
pub use node::AstNode;
pub use sub::{Param, Sub};
pub use tok_name_for_error::tok_name_for_error;
pub use token::Token;

use crate::error::Error;
use crate::parser::ast::AstResolver;

pub fn fill_executor(resolver: &mut AstResolver<AstNode>) {
    special_ast_nodes::fill_executor(resolver);
    ecl::fill_executor(resolver);
    sub::fill_executor(resolver);
    instr::fill_executor(resolver);
    expr::fill_executor(resolver);
}
