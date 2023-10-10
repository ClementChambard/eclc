use super::*;

#[derive(Debug, Clone)]
pub enum TimeLabelKind {
    Add,
    Set,
    Sub,
}

#[derive(Debug, Clone)]
pub enum Instr {
    Label(String),
    TimeLabel(i32, TimeLabelKind),
    RankLabel(u8),
    Call(String, Vec<Expr>),
    Bloc(Vec<Instr>),
    PushExpr(Expr),
    If(Expr, Vec<Instr>, Vec<Instr>),
    Loop(Vec<Instr>),
    While(Expr, Vec<Instr>),
    DoWhile(Expr, Vec<Instr>),
    Affect(String, Expr),
    VarInt(String, Option<Expr>),
    VarFloat(String, Option<Expr>),
    Break,
    Continue,
}

impl Instr {
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
                            let slen = st.len();
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

fn resolve_instr(typ: &Vec<String>, args: &Vec<AstNode>) -> AstNode {
    assert!(typ.len() == 1 || typ.len() == 2);
    let typ0 = &typ[0];
    AstNode::Instr(match &typ0[..] {
        "None" => return AstNode::None,
        "InstrSub" => {
            assert!(args.len() == 2);
            let id = args[0].clone().token().id();
            let (dtype, children) = args[1].clone().data();
            match &dtype[..] {
                "InstrSub::Call" => {
                    assert!(children.len() == 1);
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
            let id = args[0].clone().token().id();
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
                    -1
                } else {
                    print!("{:?}", asy[0]);
                    asy[0].clone().token().int()
                }
            } else {
                -1
            };
            params.insert(0, Expr::Str(id));
            let ins_call = if is_async {
                if async_num >= 0 {
                    params.insert(1, Expr::Int(async_num));
                    "ins_16"
                } else {
                    "ins_15"
                }
            } else {
                "ins_11"
            };
            Instr::Call(ins_call.to_string(), params)
        }
        "TimeLabel" => {
            assert!(args.len() == 1);
            let typ1 = &typ[1];
            match &typ1[..] {
                "Set" => Instr::TimeLabel(args[0].clone().token().int(), TimeLabelKind::Set),
                "Add" => Instr::TimeLabel(args[0].clone().token().int(), TimeLabelKind::Add),
                "Sub" => Instr::TimeLabel(args[0].clone().token().int(), TimeLabelKind::Sub),
                f => panic!("Unknown TimeLabelKind {}", f),
            }
        }
        "RankLabel" => {
            let typ1 = &typ[1];
            match &typ1[..] {
                "Spec" => {
                    let id = args[0].clone().token().id();
                    let mut rk = 192u8;
                    for c in id.chars() {
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
                    Instr::RankLabel(rk)
                }
                "All" => Instr::RankLabel(255u8),
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
                "ins_12".to_string(),
                vec![
                    Expr::Id(args[0].clone().token().id()),
                    Expr::Int(args[1].clone().token().int()),
                ],
            )
        }
        "Break" => {
            assert!(args.len() == 0);
            Instr::Break
        }
        "Continue" => {
            assert!(args.len() == 0);
            Instr::Continue
        }
        "Return" => {
            assert!(args.len() == 0);
            Instr::Call("ins_10".to_string(), vec![])
        }
        "Delete" => {
            assert!(args.len() == 0);
            Instr::Call("ins_1".to_string(), vec![])
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
                    return AstNode::Data {
                        dtype: "Else::Some".to_string(),
                        children: args.clone(),
                    };
                }
                "None" => {
                    assert!(args.len() == 0);
                    return AstNode::Data {
                        dtype: "Else::None".to_string(),
                        children: vec![],
                    };
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
            let e = if children.len() == 0 {
                None
            } else {
                Some(children[0].clone().expr())
            };
            Instr::VarInt(args[0].clone().token().id(), e)
        }
        "NewVarFloat" => {
            assert!(args.len() == 2);
            let (_, children) = args[1].clone().data();
            let e = if children.len() == 0 {
                None
            } else {
                Some(children[0].clone().expr())
            };
            Instr::VarFloat(args[0].clone().token().id(), e)
        }
        f => {
            panic!("Unknown Instr command {f}");
        }
    })
}

fn resolve_instrsub(typ: &Vec<String>, args: &Vec<AstNode>) -> AstNode {
    assert!(typ.len() == 1);
    let typ = &typ[0];
    assert!(
        &typ[..] == "Call"
            || &typ[..] == "Label"
            || &typ[..] == "Affect"
            || &typ[..] == "None"
            || &typ[..] == "Async"
    );
    AstNode::Data {
        dtype: format!("InstrSub::{}", &typ[..]),
        children: args.to_vec(),
    }
}

pub fn fill_executor(resolver: &mut AstResolver<AstNode>) {
    resolver.add_func("Instr", resolve_instr);
    resolver.add_func("InstrSub", resolve_instrsub);
}
