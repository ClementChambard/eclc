mod tokens_to_vals;

mod builtin_idents;

mod if_construct;
mod loop_construct;
mod while_construct;

mod expr;
pub use expr::{Expr, ExprType};

mod instr;
pub use instr::{Instr, TimeLabelKind};

use crate::parser::ast::{AstResolver, NeededForAstNode};
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
        match self {
            Self::None => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Ecl {
    pub ecli: Vec<String>,
    pub anmi: Vec<String>,
    pub subs: Vec<Sub>,
}

impl Ecl {
    pub fn process(&mut self) {
        for s in &mut self.subs {
            s.process();
        }
    }
}

#[derive(Debug, Clone)]
pub enum Param {
    Int(String),
    Float(String),
}

#[derive(Debug, Clone)]
pub struct Sub {
    pub name: String,
    pub params: Vec<Param>,
    pub instructions: Vec<Instr>,
}

impl Sub {
    fn gen_label(&self, lbl_seed: &mut usize) -> String {
        let mut n = self.name.clone();
        n.push_str(&format!("_label_{}", lbl_seed));
        *lbl_seed += 1;
        n
    }

    fn flatten_bloc(bloc: &Vec<Instr>) -> Vec<Instr> {
        let mut new_instructions = Vec::new();
        for i in bloc {
            match i {
                Instr::Bloc(l) => new_instructions.extend(Self::flatten_bloc(l)),
                _ => new_instructions.push(i.clone()),
            }
        }
        new_instructions
    }

    fn resolve_push_expr(&mut self) {
        let mut new_instructions = Vec::new();
        for i in &self.instructions {
            match i {
                Instr::PushExpr(e) => new_instructions.extend(e.instructions()),
                _ => new_instructions.push(i.clone()),
            }
        }
        self.instructions = new_instructions;
    }

    fn check_expressions(&mut self) {
        let mut new_instructions = Vec::new();
        for i in &self.instructions {
            match i {
                Instr::PushExpr(e) => {
                    let mut e = e.clone();
                    e.anotate();
                    e.constant_fold();
                    new_instructions.push(Instr::PushExpr(e));
                }
                Instr::Call(name, v) => {
                    let mut args = Vec::new();
                    let mut stoff = -1;
                    for e in v {
                        let mut e = e.clone();
                        e.anotate();
                        e.constant_fold();
                        if e.is_primitive() {
                            args.push(e);
                        } else {
                            let t = e.get_type();
                            new_instructions.push(Instr::PushExpr(e));
                            match t {
                                ExprType::Int => args.push(Expr::VarInt(stoff)),
                                ExprType::Float => args.push(Expr::VarFloat(stoff as f32)),
                                _ => panic!("Can't push non number onto the stack"),
                            }
                            stoff -= 1;
                        }
                    }
                    // check parameter types
                    // if vararg, insert type markers
                    new_instructions.push(Instr::Call(name.clone(), args));
                }
                _ => new_instructions.push(i.clone()),
            }
        }
        self.instructions = new_instructions;
    }

    fn check_if_sub_returns(&mut self) {
        // CAREFUL: a jump instruction could jump over a return ?
        let last = self.instructions.iter().last();
        if let Some(last) = last {
            match last {
                Instr::Call(name, _) => {
                    let opcode = crate::code_gen::resolve_ins_opcode(&name);
                    if opcode != 10 && opcode != 1 {
                        self.instructions
                            .push(Instr::Call("ins_10".to_string(), vec![]));
                    }
                }
                _ => self
                    .instructions
                    .push(Instr::Call("ins_10".to_string(), vec![])),
            }
        } else {
            self.instructions
                .push(Instr::Call("ins_10".to_string(), vec![]));
        }
    }

    pub fn process(&mut self) {
        let mut lbl_seed = 0usize;
        self.instructions = builtin_idents::replace(&self.instructions);
        self.instructions = if_construct::desugar_bloc(&self, &self.instructions, &mut lbl_seed);
        self.instructions = loop_construct::desugar_bloc(&self, &self.instructions, &mut lbl_seed);
        self.instructions = while_construct::desugar_bloc(&self, &self.instructions, &mut lbl_seed);
        // desugar other
        // maybe resolve variables before flattening anything.
        self.instructions = Self::flatten_bloc(&self.instructions);
        self.check_if_sub_returns();
        self.check_expressions();
        self.resolve_push_expr();
        self.resolve_labels();
        // optimize jump chain and remove dead code at some point
        // resolve other identifiers: vars, constants ... (right now there is none)
    }

    fn resolve_labels(&mut self) {
        let mut labels = std::collections::HashMap::new();
        let mut new_instructions = vec![];
        let mut pos = 0;
        for i in &self.instructions {
            match i {
                Instr::Label(lbl) => {
                    labels.insert(lbl.clone(), Expr::Int(pos as i32));
                }
                _ => new_instructions.push(i.clone()),
            }
            pos += i.size();
        }
        for ni in &mut new_instructions {
            match ni {
                Instr::Call(_, v) => {
                    for e in v.iter_mut() {
                        e.replace_all_id(&labels);
                    }
                }
                _ => {}
            }
        }
        self.instructions = new_instructions;
    }
}

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

impl NeededForAstNode for AstNode {
    fn from_token(tok: &crate::lexer::Token<&str>) -> Self {
        Self::Token(Token::from(tok))
    }
}

pub fn resolve_ecl(typ: &Vec<String>, args: &Vec<AstNode>) -> AstNode {
    assert!(typ.is_empty());
    assert!(args.len() == 3);
    let ecli = args[0]
        .clone()
        .list()
        .into_iter()
        .map(|n| n.token().strn())
        .collect();
    let anmi = args[1]
        .clone()
        .list()
        .into_iter()
        .map(|n| n.token().strn())
        .collect();
    let subs = args[2]
        .clone()
        .list()
        .into_iter()
        .map(|n| n.sub())
        .collect();
    AstNode::Ecl(Ecl { ecli, anmi, subs })
}

pub fn resolve_list(typ: &Vec<String>, args: &Vec<AstNode>) -> AstNode {
    assert!(typ.len() == 1);
    let typ = &typ[0];
    match &typ[..] {
        "empty" => AstNode::List(vec![]),
        "prepend" => {
            assert!(args.len() == 2);
            let mut list = args[0].clone().list();
            let val = args[1].clone();
            if !val.is_none() {
                list.insert(0, val);
            }
            AstNode::List(list)
        }
        f => {
            panic!("Unknown list function {f}");
        }
    }
}

pub fn resolve_param(typ: &Vec<String>, args: &Vec<AstNode>) -> AstNode {
    assert!(typ.len() == 1);
    assert!(args.len() == 1);
    let typ = &typ[0];
    AstNode::Param(match &typ[..] {
        "Int" => Param::Int(args[0].clone().token().id()),
        "Float" => Param::Float(args[0].clone().token().id()),
        _ => panic!("Unknown param type"),
    })
}

pub fn resolve_sub(typ: &Vec<String>, args: &Vec<AstNode>) -> AstNode {
    assert!(typ.is_empty());
    assert!(args.len() == 3);
    let val = args[0].clone().token().id();
    let param_list = args[1].clone().list();
    let ins_list = args[2].clone().list();
    AstNode::Sub(Sub {
        name: val.clone(),
        params: param_list.into_iter().map(|n| n.param()).collect(),
        instructions: ins_list.into_iter().map(|n| n.instr()).collect(),
    })
}

pub fn fill_executor(resolver: &mut AstResolver<AstNode>) {
    resolver.add_func("List", resolve_list);
    resolver.add_func("Ecl", resolve_ecl);
    resolver.add_func("Sub", resolve_sub);
    instr::fill_executor(resolver);
    expr::fill_executor(resolver);
    resolver.add_func("Param", resolve_param);
}
