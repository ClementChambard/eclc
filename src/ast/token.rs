use super::*;

use magic_unwrapper::EnumUnwrap;

#[derive(Debug, Clone, EnumUnwrap)]
pub enum Token {
    Strn(String),
    Int(i32),
    Float(f32),
    Id(String),
    Other(String),
}

impl From<&crate::lexer::Token<'_, &str>> for Token {
    fn from(value: &crate::lexer::Token<&str>) -> Self {
        match value.kind {
            "id" => Self::Id(value.text.to_string()),
            "int" => Self::Int(tokens_to_vals::int(value.text)),
            "float" => Self::Float(tokens_to_vals::float(value.text)),
            "str" => Self::Strn(tokens_to_vals::string(value.text)),
            _ => Self::Other(value.kind.to_string()),
        }
    }
}
