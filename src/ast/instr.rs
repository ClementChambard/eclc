use crate::error::{report_error_ext, Error};

use super::*;

#[derive(Debug, Clone)]
pub enum TimeLabelKind {
    Add,
    Set,
    Sub,
}

#[derive(Debug, Clone)]
pub enum Instr {
    Label(Located<String>),
    TimeLabel(Located<i32>, TimeLabelKind),
    RankLabel(Located<u8>),
    Call(Located<String>, Vec<Expr>),
    Bloc(Vec<Instr>),
    PushExpr(Expr),
    If(Expr, Vec<Instr>, Vec<Instr>),
    Loop(Vec<Instr>),
    While(Expr, Vec<Instr>),
    DoWhile(Expr, Vec<Instr>),
    Affect(Located<String>, Expr),
    VarInt(Located<String>, Option<Expr>),
    VarFloat(Located<String>, Option<Expr>),
    Break,
    Continue,
}

impl Instr {
    pub fn signature(&self) -> Result<String, Error> {
        match self {
            Self::Call(n, e) => {
                let mut s = n.val().clone();
                s.push('(');
                for (i, ex) in e.iter().enumerate() {
                    if i != 0 {
                        s.push_str(", ");
                    }
                    match ex.get_type().unwrap() {
                        ExprType::Int => s.push_str("int"),
                        ExprType::Float => s.push_str("float"),
                        ExprType::String => s.push_str("str"),
                        ExprType::Vararg => s.push_str("..."),
                    }
                }
                s.push(')');
                Ok(s)
            }
            _ => Err(Error::BackEnd(
                "Can't get signature of non inscall Instr".to_owned(),
            )),
        }
    }

    pub fn size(&self) -> usize {
        match self {
            Self::Label(_) => 0,
            Self::TimeLabel(_, _) => 0,
            Self::RankLabel(_) => 0,
            Self::Call(_, v) => {
                let mut s = 16;
                for e in v {
                    s += match e {
                        Expr::Str(st) => {
                            let slen = st.val().len();
                            slen + 4 + (4 - (slen % 4))
                        }
                        _ => 4,
                    };
                }
                s
            }
            Self::Bloc(v) => v.iter().map(|i| i.size()).sum(),
            _ => panic!("Should not call size on this Instr type"),
        }
    }
}

fn resolve_instr(typ: &[String], args: &[AstNode]) -> Result<AstNode, Error> {
    assert!(typ.len() == 1 || typ.len() == 2);
    let typ0 = &typ[0];
    Ok(AstNode::Instr(match &typ0[..] {
        "None" => return Ok(AstNode::None),
        "InstrSub" => {
            assert!(args.len() == 2);
            let id = args[0].clone().token().id_loc();
            let (dtype, children) = args[1].clone().data();
            match &dtype[..] {
                "InstrSub::Call" => {
                    assert!(children.len() == 1);
                    if id.val().starts_with("ins_") {
                        let num = id.val().strip_prefix("ins_").unwrap();
                        match num.parse::<u16>() {
                            Ok(_) => {}
                            Err(_) => {
                                report_error_ext(
                                    id.loc(),
                                    &format!("instruction `{}` does not exist", id.val()),
                                    "unknown instruction",
                                );
                                return Err(Error::Simple("instruction does not exist".to_owned()));
                            }
                        }
                    }
                    Instr::Call(
                        id,
                        children[0]
                            .clone()
                            .list()
                            .into_iter()
                            .map(|n| n.expr())
                            .collect(),
                    )
                }
                "InstrSub::Label" => Instr::Label(id),
                "InstrSub::Affect" => {
                    assert!(children.len() == 1);
                    Instr::Affect(id, children[0].clone().expr())
                }
                f => {
                    panic!("Unknown: {}", f)
                }
            }
        }
        "SubCall" => {
            assert!(args.len() == 3);
            let id = args[0].clone().token().id_loc();
            let mut params: Vec<_> = args[1]
                .clone()
                .list()
                .into_iter()
                .map(|n| n.expr())
                .collect();
            let (_, children) = args[2].clone().data();
            let is_async = !children.is_empty();
            let async_num = if is_async {
                let (_, asy) = children[0].clone().data();
                if asy.is_empty() {
                    (-1).into()
                } else {
                    asy[0].clone().token().int_loc()
                }
            } else {
                (-1).into()
            };
            params.insert(0, Expr::Str(id.clone()));
            let ins_call = if is_async {
                if *async_num.val() >= 0 {
                    params.insert(1, Expr::Int(async_num));
                    "ins_16"
                } else {
                    "ins_15"
                }
            } else {
                "ins_11"
            };
            Instr::Call(Located::new(ins_call.to_string(), id.loc().clone()), params)
        }
        "TimeLabel" => {
            assert!(args.len() == 1);
            let typ1 = &typ[1];
            match &typ1[..] {
                "Set" => Instr::TimeLabel(args[0].clone().token().int_loc(), TimeLabelKind::Set),
                "Add" => Instr::TimeLabel(args[0].clone().token().int_loc(), TimeLabelKind::Add),
                "Sub" => Instr::TimeLabel(args[0].clone().token().int_loc(), TimeLabelKind::Sub),
                f => panic!("Unknown TimeLabelKind {}", f),
            }
        }
        "RankLabel" => {
            let typ1 = &typ[1];
            match &typ1[..] {
                "Spec" => {
                    let id = args[0].clone().token().id_loc();
                    let mut rk = 192u8;
                    for c in id.val().chars() {
                        match c {
                            'e' => rk |= 1,
                            'n' => rk |= 2,
                            'h' => rk |= 4,
                            'l' => rk |= 8,
                            'x' => rk |= 16,
                            'o' => rk |= 32,
                            _ => panic!("Unknown rank character {c}"),
                        }
                    }
                    Instr::RankLabel(Located::new(rk, id.loc().clone()))
                }
                "All" => Instr::RankLabel(255u8.into()),
                f => panic!("Unknown RankLabelKind {}", f),
            }
        }
        "Bloc" => {
            assert!(args.len() == 1);
            Instr::Bloc(
                args[0]
                    .clone()
                    .list()
                    .into_iter()
                    .map(|n| n.instr())
                    .collect(),
            )
        }
        "Goto" => {
            assert!(args.len() == 2);
            Instr::Call(
                "ins_12".to_string().into(),
                vec![
                    Expr::Id(args[0].clone().token().id_loc()),
                    Expr::Float(args[1].clone().token().num_as_float_loc()),
                ],
            )
        }
        "Break" => {
            assert!(args.is_empty());
            Instr::Break
        }
        "Continue" => {
            assert!(args.is_empty());
            Instr::Continue
        }
        "Return" => {
            assert!(args.is_empty());
            Instr::Call("ins_10".to_string().into(), vec![])
        }
        "Delete" => {
            assert!(args.is_empty());
            Instr::Call("ins_1".to_string().into(), vec![])
        }
        "If" => {
            assert!(args.len() == 3);
            let cond = args[0].clone().expr();
            let if_bloc = args[1]
                .clone()
                .list()
                .into_iter()
                .map(|n| n.instr())
                .collect();
            let (dtype, children) = args[2].clone().data();
            let else_bloc = match &dtype[..] {
                "Else::Some" => {
                    let c0 = children[0].clone().instr();
                    match c0 {
                        Instr::Bloc(l) => l,
                        _ => vec![c0],
                    }
                }
                "Else::None" => vec![],
                _ => panic!(),
            };
            Instr::If(cond, if_bloc, else_bloc)
        }
        "Else" => {
            assert!(typ.len() == 2);
            let typ1 = &typ[1];
            match &typ1[..] {
                "Some" => {
                    assert!(args.len() == 1);
                    return Ok(AstNode::Data {
                        dtype: "Else::Some".to_string(),
                        children: args.to_vec(),
                    });
                }
                "None" => {
                    assert!(args.is_empty());
                    return Ok(AstNode::Data {
                        dtype: "Else::None".to_string(),
                        children: vec![],
                    });
                }
                f => panic!("Unknown Else command {f}"),
            }
        }
        "Loop" => {
            assert!(args.len() == 1);
            Instr::Loop(
                args[0]
                    .clone()
                    .list()
                    .into_iter()
                    .map(|v| v.instr())
                    .collect(),
            )
        }
        "While" => {
            assert!(args.len() == 2);
            Instr::While(
                args[0].clone().expr(),
                args[1]
                    .clone()
                    .list()
                    .into_iter()
                    .map(|v| v.instr())
                    .collect(),
            )
        }
        "DoWhile" => {
            assert!(args.len() == 2);
            Instr::DoWhile(
                args[0].clone().expr(),
                args[1]
                    .clone()
                    .list()
                    .into_iter()
                    .map(|v| v.instr())
                    .collect(),
            )
        }
        "NewVarInt" => {
            assert!(args.len() == 2);
            let (_, children) = args[1].clone().data();
            let e = if children.is_empty() {
                None
            } else {
                Some(children[0].clone().expr())
            };
            Instr::VarInt(args[0].clone().token().id_loc(), e)
        }
        "NewVarFloat" => {
            assert!(args.len() == 2);
            let (_, children) = args[1].clone().data();
            let e = if children.is_empty() {
                None
            } else {
                Some(children[0].clone().expr())
            };
            Instr::VarFloat(args[0].clone().token().id_loc(), e)
        }
        f => {
            return Err(Error::Grammar(format!("Unknown Instr command {f}")));
        }
    }))
}

fn resolve_instrsub(typ: &[String], args: &[AstNode]) -> Result<AstNode, Error> {
    if typ.len() != 1 {
        return Err(Error::Grammar("InstrSub takes 1 subcommand".to_owned()));
    }
    let typ = &typ[0];
    if !(&typ[..] == "Call"
        || &typ[..] == "Label"
        || &typ[..] == "Affect"
        || &typ[..] == "None"
        || &typ[..] == "Async")
    {
        return Err(Error::Grammar(format!("InstrSub unknown subcommand {typ}")));
    }
    Ok(AstNode::Data {
        dtype: format!("InstrSub::{}", &typ[..]),
        children: args.to_vec(),
    })
}

pub fn fill_executor(resolver: &mut AstResolver<AstNode>) {
    resolver.add_func("Instr", resolve_instr);
    resolver.add_func("InstrSub", resolve_instrsub);
}
