use super::*;

use magic_unwrapper::EnumUnwrap;

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum ExprType {
    Int,
    Float,
    String,
}

#[derive(Debug, Clone)]
pub struct ExprAnnotation {
    expr_type: ExprType,
}

#[derive(Debug, Clone, EnumUnwrap)]
pub enum Expr {
    Int(i32),
    Float(f32),
    Str(String),
    Id(String),
    VarInt(i32),
    VarFloat(f32),
    Add(Box<Expr>, Box<Expr>, Option<ExprAnnotation>),
    Sub(Box<Expr>, Box<Expr>, Option<ExprAnnotation>),
    Mul(Box<Expr>, Box<Expr>, Option<ExprAnnotation>),
    Div(Box<Expr>, Box<Expr>, Option<ExprAnnotation>),
    Modulo(Box<Expr>, Box<Expr>, Option<ExprAnnotation>),
    Gt(Box<Expr>, Box<Expr>, Option<ExprAnnotation>),
    Ge(Box<Expr>, Box<Expr>, Option<ExprAnnotation>),
    Lt(Box<Expr>, Box<Expr>, Option<ExprAnnotation>),
    Le(Box<Expr>, Box<Expr>, Option<ExprAnnotation>),
    Eq(Box<Expr>, Box<Expr>, Option<ExprAnnotation>),
    Ne(Box<Expr>, Box<Expr>, Option<ExprAnnotation>),
    BinAnd(Box<Expr>, Box<Expr>, Option<ExprAnnotation>),
    BinOr(Box<Expr>, Box<Expr>, Option<ExprAnnotation>),
    Xor(Box<Expr>, Box<Expr>, Option<ExprAnnotation>),
    Or(Box<Expr>, Box<Expr>, Option<ExprAnnotation>),
    And(Box<Expr>, Box<Expr>, Option<ExprAnnotation>),
    Sin(Box<Expr>, Option<ExprAnnotation>),
    Cos(Box<Expr>, Option<ExprAnnotation>),
    Sqrt(Box<Expr>, Option<ExprAnnotation>),
    Uminus(Box<Expr>, Option<ExprAnnotation>),
    Not(Box<Expr>, Option<ExprAnnotation>),
}

impl Expr {
    pub fn constant_fold(&mut self) {
        match self {
            Self::Add(l, r, _) => {
                l.constant_fold();
                r.constant_fold();
                match (l.as_ref(), r.as_ref()) {
                    (Expr::Int(i), Expr::Int(j)) => *self = Expr::Int(i + j),
                    (Expr::Float(i), Expr::Float(j)) => *self = Expr::Float(i + j),
                    (Expr::Str(i), Expr::Str(j)) => {
                        *self = Expr::Str({
                            let mut k = i.clone();
                            k.push_str(j);
                            k
                        })
                    }
                    _ => {}
                }
            }
            Self::Sub(l, r, _) => {
                l.constant_fold();
                r.constant_fold();
                match (l.as_ref(), r.as_ref()) {
                    (Expr::Int(i), Expr::Int(j)) => *self = Expr::Int(i - j),
                    (Expr::Float(i), Expr::Float(j)) => *self = Expr::Float(i - j),
                    _ => {}
                }
            }
            Self::Mul(l, r, _) => {
                l.constant_fold();
                r.constant_fold();
                match (l.as_ref(), r.as_ref()) {
                    (Expr::Int(i), Expr::Int(j)) => *self = Expr::Int(i * j),
                    (Expr::Float(i), Expr::Float(j)) => *self = Expr::Float(i * j),
                    (Expr::Int(i), Expr::Str(s)) | (Expr::Str(s), Expr::Int(i)) => {
                        let mut new_s = String::new();
                        for _ in 0..*i {
                            new_s.push_str(&s);
                        }
                        *self = Expr::Str(new_s)
                    }
                    _ => {}
                }
            }
            Self::Div(l, r, _) => {
                l.constant_fold();
                r.constant_fold();
                match (l.as_ref(), r.as_ref()) {
                    (Expr::Int(i), Expr::Int(j)) => *self = Expr::Int(i / j),
                    (Expr::Float(i), Expr::Float(j)) => *self = Expr::Float(i / j),
                    _ => {}
                }
            }
            Self::Modulo(l, r, _) => {
                l.constant_fold();
                r.constant_fold();
                match (l.as_ref(), r.as_ref()) {
                    (Expr::Int(i), Expr::Int(j)) => *self = Expr::Int(i % j),
                    _ => {}
                }
            }
            Self::Uminus(c, _) => {
                c.constant_fold();
                match c.as_ref() {
                    Expr::Int(i) => *self = Expr::Int(-i),
                    Expr::Float(i) => *self = Expr::Float(-i),
                    _ => {}
                }
            }
            Self::Not(c, _) => {
                c.constant_fold();
                match c.as_ref() {
                    Expr::Int(i) => *self = Expr::Int((*i == 0) as i32),
                    Expr::Float(f) => *self = Expr::Int((*f == 0.) as i32),
                    _ => {}
                }
            }
            Self::Sin(e, _) => {
                e.constant_fold();
                match e.as_ref() {
                    Expr::Float(f) => *self = Expr::Float(f.sin()),
                    _ => {}
                }
            }
            Self::Cos(e, _) => {
                e.constant_fold();
                match e.as_ref() {
                    Expr::Float(f) => *self = Expr::Float(f.cos()),
                    _ => {}
                }
            }
            Self::Sqrt(e, _) => {
                e.constant_fold();
                match e.as_ref() {
                    Expr::Float(f) => *self = Expr::Float(f.sqrt()),
                    _ => {}
                }
            }
            Self::Gt(l, r, _) => {
                l.constant_fold();
                r.constant_fold();
                match (l.as_ref(), r.as_ref()) {
                    (Expr::Float(i), Expr::Float(j)) => *self = Expr::Int((*i > *j) as i32),
                    (Expr::Int(i), Expr::Int(j)) => *self = Expr::Int((*i > *j) as i32),
                    _ => {}
                }
            }
            Self::Ge(l, r, _) => {
                l.constant_fold();
                r.constant_fold();
                match (l.as_ref(), r.as_ref()) {
                    (Expr::Float(i), Expr::Float(j)) => *self = Expr::Int((*i >= *j) as i32),
                    (Expr::Int(i), Expr::Int(j)) => *self = Expr::Int((*i >= *j) as i32),
                    _ => {}
                }
            }
            Self::Lt(l, r, _) => {
                l.constant_fold();
                r.constant_fold();
                match (l.as_ref(), r.as_ref()) {
                    (Expr::Float(i), Expr::Float(j)) => *self = Expr::Int((*i < *j) as i32),
                    (Expr::Int(i), Expr::Int(j)) => *self = Expr::Int((*i < *j) as i32),
                    _ => {}
                }
            }
            Self::Le(l, r, _) => {
                l.constant_fold();
                r.constant_fold();
                match (l.as_ref(), r.as_ref()) {
                    (Expr::Float(i), Expr::Float(j)) => *self = Expr::Int((*i <= *j) as i32),
                    (Expr::Int(i), Expr::Int(j)) => *self = Expr::Int((*i <= *j) as i32),
                    _ => {}
                }
            }
            Self::Eq(l, r, _) => {
                l.constant_fold();
                r.constant_fold();
                match (l.as_ref(), r.as_ref()) {
                    (Expr::Float(i), Expr::Float(j)) => *self = Expr::Int((*i == *j) as i32),
                    (Expr::Int(i), Expr::Int(j)) => *self = Expr::Int((*i == *j) as i32),
                    (Expr::Str(s1), Expr::Str(s2)) => *self = Expr::Int((s1 == s2) as i32),
                    _ => {}
                }
            }
            Self::Ne(l, r, _) => {
                l.constant_fold();
                r.constant_fold();
                match (l.as_ref(), r.as_ref()) {
                    (Expr::Float(i), Expr::Float(j)) => *self = Expr::Int((*i != *j) as i32),
                    (Expr::Int(i), Expr::Int(j)) => *self = Expr::Int((*i != *j) as i32),
                    (Expr::Str(s1), Expr::Str(s2)) => *self = Expr::Int((s1 != s2) as i32),
                    _ => {}
                }
            }
            Self::BinAnd(l, r, _) => {
                l.constant_fold();
                r.constant_fold();
                match (l.as_ref(), r.as_ref()) {
                    (Expr::Int(i), Expr::Int(j)) => *self = Expr::Int((*i & *j) as i32),
                    _ => {}
                }
            }
            Self::BinOr(l, r, _) => {
                l.constant_fold();
                r.constant_fold();
                match (l.as_ref(), r.as_ref()) {
                    (Expr::Int(i), Expr::Int(j)) => *self = Expr::Int((*i | *j) as i32),
                    _ => {}
                }
            }
            Self::Xor(l, r, _) => {
                l.constant_fold();
                r.constant_fold();
                match (l.as_ref(), r.as_ref()) {
                    (Expr::Int(i), Expr::Int(j)) => *self = Expr::Int((*i ^ *j) as i32),
                    _ => {}
                }
            }
            Self::Or(l, r, _) => {
                l.constant_fold();
                r.constant_fold();
                match (l.as_ref(), r.as_ref()) {
                    (Expr::Int(i), Expr::Int(j)) => {
                        *self = Expr::Int(((*i != 0) || (*j != 0)) as i32)
                    }
                    _ => {}
                }
            }
            Self::And(l, r, _) => {
                l.constant_fold();
                r.constant_fold();
                match (l.as_ref(), r.as_ref()) {
                    (Expr::Int(i), Expr::Int(j)) => {
                        *self = Expr::Int(((*i != 0) && (*j != 0)) as i32)
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    pub fn get_type(&self) -> ExprType {
        match self {
            Expr::Int(_) => ExprType::Int,
            Expr::VarInt(_) => ExprType::Int,
            Expr::Float(_) => ExprType::Float,
            Expr::VarFloat(_) => ExprType::Float,
            Expr::Str(_) => ExprType::String,
            Expr::Add(_, _, Some(a))
            | Expr::Sub(_, _, Some(a))
            | Expr::Mul(_, _, Some(a))
            | Expr::Div(_, _, Some(a))
            | Expr::Modulo(_, _, Some(a))
            | Expr::Gt(_, _, Some(a))
            | Expr::Ge(_, _, Some(a))
            | Expr::Lt(_, _, Some(a))
            | Expr::Le(_, _, Some(a))
            | Expr::Eq(_, _, Some(a))
            | Expr::Ne(_, _, Some(a))
            | Expr::BinAnd(_, _, Some(a))
            | Expr::BinOr(_, _, Some(a))
            | Expr::Xor(_, _, Some(a))
            | Expr::Or(_, _, Some(a))
            | Expr::And(_, _, Some(a))
            | Expr::Uminus(_, Some(a))
            | Expr::Not(_, Some(a))
            | Expr::Sin(_, Some(a))
            | Expr::Cos(_, Some(a))
            | Expr::Sqrt(_, Some(a)) => a.expr_type,
            Expr::Id(_) => ExprType::Int, // unaffected id can only be label at this point so it's
            // an int
            _ => panic!("Can't know type of unanotated node"),
        }
    }

    pub fn anotate(&mut self) {
        match self {
            Expr::Add(l, r, ref mut a)
            | Expr::Sub(l, r, ref mut a)
            | Expr::Mul(l, r, ref mut a)
            | Expr::Div(l, r, ref mut a) => {
                l.anotate();
                r.anotate();
                let l_type = l.get_type();
                let r_type = r.get_type();
                assert!(l_type == r_type);
                *a = Some(ExprAnnotation { expr_type: l_type })
            }
            Expr::Modulo(l, r, ref mut a)
            | Expr::BinAnd(l, r, ref mut a)
            | Expr::BinOr(l, r, ref mut a)
            | Expr::Xor(l, r, ref mut a)
            | Expr::And(l, r, ref mut a)
            | Expr::Or(l, r, ref mut a) => {
                l.anotate();
                r.anotate();
                let l_type = l.get_type();
                let r_type = r.get_type();
                assert!(l_type == r_type);
                assert!(l_type == ExprType::Int);
                *a = Some(ExprAnnotation { expr_type: l_type })
            }
            Expr::Gt(l, r, ref mut a)
            | Expr::Ge(l, r, ref mut a)
            | Expr::Lt(l, r, ref mut a)
            | Expr::Le(l, r, ref mut a)
            | Expr::Eq(l, r, ref mut a)
            | Expr::Ne(l, r, ref mut a) => {
                l.anotate();
                r.anotate();
                let l_type = l.get_type();
                let r_type = r.get_type();
                assert!(l_type == r_type);
                *a = Some(ExprAnnotation {
                    expr_type: ExprType::Int,
                })
            }
            Expr::Uminus(l, ref mut a) => {
                l.anotate();
                *a = Some(ExprAnnotation {
                    expr_type: l.get_type(),
                })
            }
            Expr::Not(l, ref mut a) => {
                l.anotate();
                *a = Some(ExprAnnotation {
                    expr_type: ExprType::Int,
                })
            }
            Expr::Sqrt(l, ref mut a) | Expr::Sin(l, ref mut a) | Expr::Cos(l, ref mut a) => {
                l.anotate();
                assert!(l.get_type() == ExprType::Float);
                *a = Some(ExprAnnotation {
                    expr_type: ExprType::Float,
                })
            }
            _ => {}
        }
    }

    pub fn instructions(&self) -> Vec<Instr> {
        let mut instructions = Vec::new();
        match self {
            Self::Int(_) => {
                instructions.push(Instr::Call("ins_42".to_string(), vec![self.clone()]));
            }
            Self::VarInt(_) => {
                instructions.push(Instr::Call("ins_42".to_string(), vec![self.clone()]));
            }
            Self::Float(_) => {
                instructions.push(Instr::Call("ins_43".to_string(), vec![self.clone()]));
            }
            Self::VarFloat(_) => {
                instructions.push(Instr::Call("ins_43".to_string(), vec![self.clone()]));
            }
            Self::Uminus(e, Some(a)) => {
                instructions.extend(e.instructions());
                match a.expr_type {
                    ExprType::Int => instructions.push(Instr::Call("ins_83".to_string(), vec![])),
                    ExprType::Float => instructions.push(Instr::Call("ins_84".to_string(), vec![])),
                    _ => panic!("Can't negate a non number"),
                }
            }
            Self::Not(e, Some(_)) => {
                instructions.extend(e.instructions());
                match e.get_type() {
                    ExprType::Int => instructions.push(Instr::Call("ins_71".to_string(), vec![])),
                    ExprType::Float => instructions.push(Instr::Call("ins_72".to_string(), vec![])),
                    _ => panic!("Can't not a non number"),
                }
            }
            Self::Add(e1, e2, Some(a)) => {
                instructions.extend(e1.instructions());
                instructions.extend(e2.instructions());
                match a.expr_type {
                    ExprType::Int => instructions.push(Instr::Call("ins_50".to_string(), vec![])),
                    ExprType::Float => instructions.push(Instr::Call("ins_51".to_string(), vec![])),
                    _ => panic!("Can't add a non number at runtime"),
                }
            }
            Self::Sub(e1, e2, Some(a)) => {
                instructions.extend(e1.instructions());
                instructions.extend(e2.instructions());
                match a.expr_type {
                    ExprType::Int => instructions.push(Instr::Call("ins_52".to_string(), vec![])),
                    ExprType::Float => instructions.push(Instr::Call("ins_53".to_string(), vec![])),
                    _ => panic!("Can't subtract a non number"),
                }
            }
            Self::Mul(e1, e2, Some(a)) => {
                instructions.extend(e1.instructions());
                instructions.extend(e2.instructions());
                match a.expr_type {
                    ExprType::Int => instructions.push(Instr::Call("ins_54".to_string(), vec![])),
                    ExprType::Float => instructions.push(Instr::Call("ins_55".to_string(), vec![])),
                    _ => panic!("Can't subtract a non number"),
                }
            }
            Self::Div(e1, e2, Some(a)) => {
                instructions.extend(e1.instructions());
                instructions.extend(e2.instructions());
                match a.expr_type {
                    ExprType::Int => instructions.push(Instr::Call("ins_56".to_string(), vec![])),
                    ExprType::Float => instructions.push(Instr::Call("ins_57".to_string(), vec![])),
                    _ => panic!("Can't subtract a non number"),
                }
            }
            Self::Gt(e1, e2, Some(a)) => {
                instructions.extend(e1.instructions());
                instructions.extend(e2.instructions());
                match a.expr_type {
                    ExprType::Int => instructions.push(Instr::Call("ins_67".to_string(), vec![])),
                    ExprType::Float => instructions.push(Instr::Call("ins_68".to_string(), vec![])),
                    _ => panic!("Can't subtract a non number"),
                }
            }
            Self::Ge(e1, e2, Some(a)) => {
                instructions.extend(e1.instructions());
                instructions.extend(e2.instructions());
                match a.expr_type {
                    ExprType::Int => instructions.push(Instr::Call("ins_69".to_string(), vec![])),
                    ExprType::Float => instructions.push(Instr::Call("ins_70".to_string(), vec![])),
                    _ => panic!("Can't subtract a non number"),
                }
            }
            Self::Lt(e1, e2, Some(a)) => {
                instructions.extend(e1.instructions());
                instructions.extend(e2.instructions());
                match a.expr_type {
                    ExprType::Int => instructions.push(Instr::Call("ins_63".to_string(), vec![])),
                    ExprType::Float => instructions.push(Instr::Call("ins_64".to_string(), vec![])),
                    _ => panic!("Can't subtract a non number"),
                }
            }
            Self::Le(e1, e2, Some(a)) => {
                instructions.extend(e1.instructions());
                instructions.extend(e2.instructions());
                match a.expr_type {
                    ExprType::Int => instructions.push(Instr::Call("ins_65".to_string(), vec![])),
                    ExprType::Float => instructions.push(Instr::Call("ins_66".to_string(), vec![])),
                    _ => panic!("Can't subtract a non number"),
                }
            }
            Self::Eq(e1, e2, Some(a)) => {
                instructions.extend(e1.instructions());
                instructions.extend(e2.instructions());
                match a.expr_type {
                    ExprType::Int => instructions.push(Instr::Call("ins_59".to_string(), vec![])),
                    ExprType::Float => instructions.push(Instr::Call("ins_60".to_string(), vec![])),
                    _ => panic!("Can't subtract a non number"),
                }
            }
            Self::Ne(e1, e2, Some(a)) => {
                instructions.extend(e1.instructions());
                instructions.extend(e2.instructions());
                match a.expr_type {
                    ExprType::Int => instructions.push(Instr::Call("ins_61".to_string(), vec![])),
                    ExprType::Float => instructions.push(Instr::Call("ins_62".to_string(), vec![])),
                    _ => panic!("Can't subtract a non number"),
                }
            }
            Self::Modulo(e1, e2, Some(_)) => {
                instructions.extend(e1.instructions());
                instructions.extend(e2.instructions());
                instructions.push(Instr::Call("ins_58".to_string(), vec![]));
            }
            Self::BinAnd(e1, e2, Some(_)) => {
                instructions.extend(e1.instructions());
                instructions.extend(e2.instructions());
                instructions.push(Instr::Call("ins_77".to_string(), vec![]));
            }
            Self::Xor(e1, e2, Some(_)) => {
                instructions.extend(e1.instructions());
                instructions.extend(e2.instructions());
                instructions.push(Instr::Call("ins_75".to_string(), vec![]));
            }
            Self::BinOr(e1, e2, Some(_)) => {
                instructions.extend(e1.instructions());
                instructions.extend(e2.instructions());
                instructions.push(Instr::Call("ins_76".to_string(), vec![]));
            }
            Self::Or(e1, e2, Some(_)) => {
                instructions.extend(e1.instructions());
                instructions.extend(e2.instructions());
                instructions.push(Instr::Call("ins_73".to_string(), vec![]));
            }
            Self::And(e1, e2, Some(_)) => {
                instructions.extend(e1.instructions());
                instructions.extend(e2.instructions());
                instructions.push(Instr::Call("ins_74".to_string(), vec![]));
            }
            Self::Sin(e, Some(_)) => {
                instructions.extend(e.instructions());
                instructions.push(Instr::Call("ins_79".to_string(), vec![]));
            }
            Self::Cos(e, Some(_)) => {
                instructions.extend(e.instructions());
                instructions.push(Instr::Call("ins_80".to_string(), vec![]));
            }
            Self::Sqrt(e, Some(_)) => {
                instructions.extend(e.instructions());
                instructions.push(Instr::Call("ins_88".to_string(), vec![]));
            }
            Self::Id(i) => panic!("Unresolved identifier {i}"),
            Self::Str(_) => panic!("Can't push a string on the stack"),
            _ => panic!(
                "Trying to generate instruction for non typed expression {:?}",
                self
            ),
        }
        instructions
    }

    pub fn is_var(&self) -> bool {
        match self {
            Self::VarInt(_) | Self::VarFloat(_) => true,
            _ => false,
        }
    }

    pub fn replace_id(&mut self, id: &str, to: &Expr) {
        match self {
            Self::Id(s) => {
                if s == id {
                    *self = to.clone();
                }
            }
            Self::Add(a, b, _)
            | Self::Sub(a, b, _)
            | Self::Mul(a, b, _)
            | Self::Div(a, b, _)
            | Self::Ne(a, b, _)
            | Self::Eq(a, b, _)
            | Self::Gt(a, b, _)
            | Self::Ge(a, b, _)
            | Self::Lt(a, b, _)
            | Self::Le(a, b, _)
            | Self::BinOr(a, b, _)
            | Self::BinAnd(a, b, _)
            | Self::Xor(a, b, _)
            | Self::Or(a, b, _)
            | Self::And(a, b, _)
            | Self::Modulo(a, b, _) => {
                a.replace_id(id, to);
                b.replace_id(id, to);
            }
            Self::Uminus(a, _)
            | Self::Not(a, _)
            | Self::Sin(a, _)
            | Self::Cos(a, _)
            | Self::Sqrt(a, _) => {
                a.replace_id(id, to);
            }
            Self::Int(_) => {}
            Self::VarInt(_) => {}
            Self::Float(_) => {}
            Self::VarFloat(_) => {}
            Self::Str(_) => {}
        }
    }

    pub fn replace_all_id(&mut self, ids: &std::collections::HashMap<String, Expr>) {
        for (id, ex) in ids {
            self.replace_id(&id, &ex);
        }
    }

    pub fn is_primitive(&self) -> bool {
        match self {
            Self::Int(_)
            | Self::Float(_)
            | Self::Str(_)
            | Self::Id(_)
            | Self::VarFloat(_)
            | Self::VarInt(_) => true,
            _ => false,
        }
    }
}

fn resolve_expr(typ: &Vec<String>, args: &Vec<AstNode>) -> AstNode {
    use Expr as E;
    assert!(typ.len() == 1);
    let typ = &typ[0];
    AstNode::Expr(match &typ[..] {
        "Int" => {
            assert!(args.len() == 1);
            E::Int(args[0].clone().token().int())
        }
        "Float" => {
            assert!(args.len() == 1);
            E::Float(args[0].clone().token().float())
        }
        "Str" => {
            assert!(args.len() == 1);
            E::Str(args[0].clone().token().strn())
        }
        "Id" => {
            assert!(args.len() == 1);
            E::Id(args[0].clone().token().id())
        }
        "Add" => {
            assert!(args.len() == 2);
            E::Add(
                Box::new(args[0].clone().expr()),
                Box::new(args[1].clone().expr()),
                None,
            )
        }
        "Sub" => {
            assert!(args.len() == 2);
            E::Sub(
                Box::new(args[0].clone().expr()),
                Box::new(args[1].clone().expr()),
                None,
            )
        }
        "Mul" => {
            assert!(args.len() == 2);
            E::Mul(
                Box::new(args[0].clone().expr()),
                Box::new(args[1].clone().expr()),
                None,
            )
        }
        "Div" => {
            assert!(args.len() == 2);
            E::Div(
                Box::new(args[0].clone().expr()),
                Box::new(args[1].clone().expr()),
                None,
            )
        }
        "Mod" => {
            assert!(args.len() == 2);
            E::Modulo(
                Box::new(args[0].clone().expr()),
                Box::new(args[1].clone().expr()),
                None,
            )
        }
        "Gt" => {
            assert!(args.len() == 2);
            E::Gt(
                Box::new(args[0].clone().expr()),
                Box::new(args[1].clone().expr()),
                None,
            )
        }
        "Ge" => {
            assert!(args.len() == 2);
            E::Ge(
                Box::new(args[0].clone().expr()),
                Box::new(args[1].clone().expr()),
                None,
            )
        }
        "Lt" => {
            assert!(args.len() == 2);
            E::Lt(
                Box::new(args[0].clone().expr()),
                Box::new(args[1].clone().expr()),
                None,
            )
        }
        "Le" => {
            assert!(args.len() == 2);
            E::Le(
                Box::new(args[0].clone().expr()),
                Box::new(args[1].clone().expr()),
                None,
            )
        }
        "Eq" => {
            assert!(args.len() == 2);
            E::Eq(
                Box::new(args[0].clone().expr()),
                Box::new(args[1].clone().expr()),
                None,
            )
        }
        "Ne" => {
            assert!(args.len() == 2);
            E::Ne(
                Box::new(args[0].clone().expr()),
                Box::new(args[1].clone().expr()),
                None,
            )
        }
        "BinOr" => {
            assert!(args.len() == 2);
            E::BinOr(
                Box::new(args[0].clone().expr()),
                Box::new(args[1].clone().expr()),
                None,
            )
        }
        "BinAnd" => {
            assert!(args.len() == 2);
            E::BinAnd(
                Box::new(args[0].clone().expr()),
                Box::new(args[1].clone().expr()),
                None,
            )
        }
        "Xor" => {
            assert!(args.len() == 2);
            E::Xor(
                Box::new(args[0].clone().expr()),
                Box::new(args[1].clone().expr()),
                None,
            )
        }
        "Or" => {
            assert!(args.len() == 2);
            E::Or(
                Box::new(args[0].clone().expr()),
                Box::new(args[1].clone().expr()),
                None,
            )
        }
        "And" => {
            assert!(args.len() == 2);
            E::And(
                Box::new(args[0].clone().expr()),
                Box::new(args[1].clone().expr()),
                None,
            )
        }
        "Uminus" => {
            assert!(args.len() == 1);
            E::Uminus(Box::new(args[0].clone().expr()), None)
        }
        "Not" => {
            assert!(args.len() == 1);
            E::Not(Box::new(args[0].clone().expr()), None)
        }
        "Sin" => {
            assert!(args.len() == 1);
            E::Sin(Box::new(args[0].clone().expr()), None)
        }
        "Cos" => {
            assert!(args.len() == 1);
            E::Cos(Box::new(args[0].clone().expr()), None)
        }
        "Sqrt" => {
            assert!(args.len() == 1);
            E::Sqrt(Box::new(args[0].clone().expr()), None)
        }
        "Var" => {
            assert!(args.len() == 1);
            let AstNode::Data{ref dtype, ref children} = args[0] else { panic!(); };
            match &dtype[..] {
                "VarExpr::Int" => {
                    assert!(children.len() == 1);
                    E::VarInt(children[0].clone().token().int())
                }
                "VarExpr::Float" => {
                    assert!(children.len() == 1);
                    E::VarFloat(children[0].clone().token().float())
                }
                _ => {
                    panic!();
                }
            }
        }
        t => {
            panic!("unknown type {t} for Expr");
        }
    })
}

fn resolve_varexpr(typ: &Vec<String>, args: &Vec<AstNode>) -> AstNode {
    let mut it = typ.iter();
    let typ = it.next().expect("VarExpr should have subtype");
    assert!(it.next().is_none());
    match &typ[..] {
        "Int" => AstNode::Data {
            dtype: "VarExpr::Int".to_string(),
            children: args.to_vec(),
        },
        "Float" => AstNode::Data {
            dtype: "VarExpr::Float".to_string(),
            children: args.to_vec(),
        },
        "MInt" => {
            let int = -args[0].clone().token().int();
            let int = AstNode::Token(Token::Int(int));
            AstNode::Data {
                dtype: "VarExpr::Int".to_string(),
                children: vec![int],
            }
        }
        "MFloat" => {
            let float = -args[0].clone().token().float();
            let float = AstNode::Token(Token::Float(float));
            AstNode::Data {
                dtype: "VarExpr::Float".to_string(),
                children: vec![float],
            }
        }
        t => {
            panic!("unknown type {t} for VarExpr");
        }
    }
}

pub fn fill_executor(resolver: &mut AstResolver<AstNode>) {
    resolver.add_func("Expr", resolve_expr);
    resolver.add_func("VarExpr", resolve_varexpr);
}
