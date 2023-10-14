use crate::lexer::Location;

use super::*;

#[derive(Debug, Clone)]
pub enum Token {
    Strn(String, Location),
    Int(i32, Location),
    Float(f32, Location),
    Id(String, Location),
    Other(String, Location),
}

impl From<&crate::lexer::Token<&str>> for Token {
    fn from(value: &crate::lexer::Token<&str>) -> Self {
        match value.kind {
            "id" => Self::Id(value.text.to_string(), value.loc.clone()),
            "int" => Self::Int(tokens_to_vals::int(&value.text), value.loc.clone()),
            "float" => Self::Float(tokens_to_vals::float(&value.text), value.loc.clone()),
            "str" => Self::Strn(tokens_to_vals::string(&value.text), value.loc.clone()),
            _ => Self::Other(value.kind.to_string(), value.loc.clone()),
        }
    }
}

impl Token {
    pub fn loc(&self) -> &Location {
        match self {
            Self::Strn(_, l) => l,
            Self::Int(_, l) => l,
            Self::Float(_, l) => l,
            Self::Id(_, l) => l,
            Self::Other(_, l) => l,
        }
    }

    pub fn strn(&self) -> String {
        if let Self::Strn(s, _) = self {
            s.clone()
        } else {
            panic!("Token is not variant Strn");
        }
    }

    pub fn strn_loc(&self) -> Located<String> {
        if let Self::Strn(s, l) = self {
            Located::new(s.clone(), l.clone())
        } else {
            panic!("Token is not variant Strn");
        }
    }

    pub fn _strn_or<T>(&self, or: T) -> Result<String, T> {
        if let Self::Strn(s, _) = self {
            Ok(s.clone())
        } else {
            Err(or)
        }
    }

    pub fn id(&self) -> String {
        if let Self::Id(s, _) = self {
            s.clone()
        } else {
            panic!("Token is not variant Id");
        }
    }

    pub fn id_loc(&self) -> Located<String> {
        if let Self::Id(s, l) = self {
            Located::new(s.clone(), l.clone())
        } else {
            panic!("Token is not variant Id");
        }
    }

    pub fn _id_or<T>(&self, or: T) -> Result<String, T> {
        if let Self::Id(s, _) = self {
            Ok(s.clone())
        } else {
            Err(or)
        }
    }

    pub fn _int(&self) -> i32 {
        if let Self::Int(s, _) = self {
            *s
        } else {
            panic!("Token is not variant Int");
        }
    }

    pub fn int_loc(&self) -> Located<i32> {
        if let Self::Int(s, l) = self {
            Located::new(*s, l.clone())
        } else {
            panic!("Token is not variant Int");
        }
    }

    pub fn int_or<T>(&self, or: T) -> Result<i32, T> {
        if let Self::Int(s, _) = self {
            Ok(*s)
        } else {
            Err(or)
        }
    }

    pub fn int_loc_or<T>(&self, or: T) -> Result<Located<i32>, T> {
        if let Self::Int(s, l) = self {
            Ok(Located::new(*s, l.clone()))
        } else {
            Err(or)
        }
    }

    pub fn _float(&self) -> f32 {
        if let Self::Float(s, _) = self {
            *s
        } else {
            panic!("Token is not variant Float");
        }
    }

    pub fn float_loc(&self) -> Located<f32> {
        if let Self::Float(s, l) = self {
            Located::new(*s, l.clone())
        } else {
            panic!("Token is not variant Float");
        }
    }

    pub fn float_or<T>(&self, or: T) -> Result<f32, T> {
        if let Self::Float(s, _) = self {
            Ok(*s)
        } else {
            Err(or)
        }
    }

    pub fn float_loc_or<T>(&self, or: T) -> Result<Located<f32>, T> {
        if let Self::Float(s, l) = self {
            Ok(Located::new(*s, l.clone()))
        } else {
            Err(or)
        }
    }

    pub fn _num_as_float(self) -> f32 {
        match self {
            Self::Int(i, _) => i as f32,
            Self::Float(f, _) => f,
            _ => panic!(""),
        }
    }

    pub fn num_as_float_loc(self) -> Located<f32> {
        match self {
            Self::Int(i, l) => Located::new(i as f32, l),
            Self::Float(f, l) => Located::new(f, l),
            _ => panic!(""),
        }
    }
}
