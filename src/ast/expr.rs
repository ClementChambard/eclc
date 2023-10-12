use crate::error::Error;

use super::*;

use magic_unwrapper::EnumUnwrap;

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum ExprType {
    Int,
    Float,
    String,
    Vararg,
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
    Vararg(Vec<Expr>),
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
            Self::Int(_) => {}
            Self::Float(_) => {}
            Self::Id(_) => {}
            Self::Str(_) => {}
            Self::VarInt(_) => {}
            Self::VarFloat(_) => {}
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
                            new_s.push_str(s);
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
                if let (Expr::Int(i), Expr::Int(j)) = (l.as_ref(), r.as_ref()) {
                    *self = Expr::Int(i % j);
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
                if let Expr::Float(f) = e.as_ref() {
                    *self = Expr::Float(f.sin());
                }
            }
            Self::Cos(e, _) => {
                e.constant_fold();
                if let Expr::Float(f) = e.as_ref() {
                    *self = Expr::Float(f.cos());
                }
            }
            Self::Sqrt(e, _) => {
                e.constant_fold();
                if let Expr::Float(f) = e.as_ref() {
                    *self = Expr::Float(f.sqrt());
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
                if let (Expr::Int(i), Expr::Int(j)) = (l.as_ref(), r.as_ref()) {
                    *self = Expr::Int(*i & *j)
                }
            }
            Self::BinOr(l, r, _) => {
                l.constant_fold();
                r.constant_fold();
                if let (Expr::Int(i), Expr::Int(j)) = (l.as_ref(), r.as_ref()) {
                    *self = Expr::Int(*i | *j)
                }
            }
            Self::Xor(l, r, _) => {
                l.constant_fold();
                r.constant_fold();
                if let (Expr::Int(i), Expr::Int(j)) = (l.as_ref(), r.as_ref()) {
                    *self = Expr::Int(*i ^ *j)
                }
            }
            Self::Or(l, r, _) => {
                l.constant_fold();
                r.constant_fold();
                if let (Expr::Int(i), Expr::Int(j)) = (l.as_ref(), r.as_ref()) {
                    *self = Expr::Int(((*i != 0) || (*j != 0)) as i32)
                }
            }
            Self::And(l, r, _) => {
                l.constant_fold();
                r.constant_fold();
                if let (Expr::Int(i), Expr::Int(j)) = (l.as_ref(), r.as_ref()) {
                    *self = Expr::Int(((*i != 0) && (*j != 0)) as i32)
                }
            }
            Self::Vararg(va) => {
                for v in va {
                    v.constant_fold();
                }
            }
        }
    }

    pub fn get_type(&self) -> Result<ExprType, Error> {
        Ok(match self {
            Expr::Int(_) => ExprType::Int,
            Expr::VarInt(_) => ExprType::Int,
            Expr::Float(_) => ExprType::Float,
            Expr::VarFloat(_) => ExprType::Float,
            Expr::Str(_) => ExprType::String,
            Expr::Vararg(_) => ExprType::Vararg,
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
            _ => {
                return Err(Error::Simple(
                    "Can't know type of unanotated node".to_owned(),
                ))
            }
        })
    }

    pub fn anotate(&mut self) -> Result<(), Error> {
        match self {
            Expr::Int(_) => {}
            Expr::Float(_) => {}
            Expr::VarInt(_) => {}
            Expr::VarFloat(_) => {}
            Expr::Str(_) => {}
            Expr::Id(_) => {}
            Expr::Vararg(ref mut va) => {
                for v in va {
                    v.anotate()?;
                }
            }
            Expr::Add(l, r, ref mut a)
            | Expr::Sub(l, r, ref mut a)
            | Expr::Mul(l, r, ref mut a)
            | Expr::Div(l, r, ref mut a) => {
                l.anotate()?;
                r.anotate()?;
                let l_type = l.get_type()?;
                let r_type = r.get_type()?;
                if l_type != r_type {
                    return Err(Error::Simple(
                        "Params of operation are expected to be the same type".to_owned(),
                    ));
                }
                *a = Some(ExprAnnotation { expr_type: l_type })
            }
            Expr::Modulo(l, r, ref mut a)
            | Expr::BinAnd(l, r, ref mut a)
            | Expr::BinOr(l, r, ref mut a)
            | Expr::Xor(l, r, ref mut a)
            | Expr::And(l, r, ref mut a)
            | Expr::Or(l, r, ref mut a) => {
                l.anotate()?;
                r.anotate()?;
                let l_type = l.get_type()?;
                let r_type = r.get_type()?;
                if l_type != r_type {
                    return Err(Error::Simple(
                        "Params of binary operation are expected to be the same type".to_owned(),
                    ));
                }
                if l_type != ExprType::Int {
                    return Err(Error::Simple(
                        "Params of binary operation are expected to be of type int".to_owned(),
                    ));
                }
                *a = Some(ExprAnnotation { expr_type: l_type })
            }
            Expr::Gt(l, r, ref mut a)
            | Expr::Ge(l, r, ref mut a)
            | Expr::Lt(l, r, ref mut a)
            | Expr::Le(l, r, ref mut a)
            | Expr::Eq(l, r, ref mut a)
            | Expr::Ne(l, r, ref mut a) => {
                l.anotate()?;
                r.anotate()?;
                let l_type = l.get_type()?;
                let r_type = r.get_type()?;
                if l_type != r_type {
                    return Err(Error::Simple(
                        "Params of comparison are expected to be the same type".to_owned(),
                    ));
                }
                *a = Some(ExprAnnotation {
                    expr_type: ExprType::Int,
                })
            }
            Expr::Uminus(l, ref mut a) => {
                l.anotate()?;
                *a = Some(ExprAnnotation {
                    expr_type: l.get_type()?,
                })
            }
            Expr::Not(l, ref mut a) => {
                l.anotate()?;
                *a = Some(ExprAnnotation {
                    expr_type: ExprType::Int,
                })
            }
            Expr::Sqrt(l, ref mut a) | Expr::Sin(l, ref mut a) | Expr::Cos(l, ref mut a) => {
                l.anotate()?;
                if l.get_type()? != ExprType::Float {
                    return Err(Error::Simple(
                        "Param of Sqrt, Sin, or Cos is expected to be a float".to_owned(),
                    ));
                }
                *a = Some(ExprAnnotation {
                    expr_type: ExprType::Float,
                })
            }
        }
        Ok(())
    }

    pub fn instructions(&self) -> Result<Vec<Instr>, Error> {
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
                instructions.extend(e.instructions()?);
                match a.expr_type {
                    ExprType::Int => instructions.push(Instr::Call("ins_83".to_string(), vec![])),
                    ExprType::Float => instructions.push(Instr::Call("ins_84".to_string(), vec![])),
                    _ => panic!("Can't negate a non number"),
                }
            }
            Self::Not(e, Some(a)) => {
                instructions.extend(e.instructions()?);
                match a.expr_type {
                    ExprType::Int => instructions.push(Instr::Call("ins_71".to_string(), vec![])),
                    ExprType::Float => instructions.push(Instr::Call("ins_72".to_string(), vec![])),
                    _ => panic!("Can't not a non number"),
                }
            }
            Self::Add(e1, e2, Some(a)) => {
                instructions.extend(e1.instructions()?);
                instructions.extend(e2.instructions()?);
                match a.expr_type {
                    ExprType::Int => instructions.push(Instr::Call("ins_50".to_string(), vec![])),
                    ExprType::Float => instructions.push(Instr::Call("ins_51".to_string(), vec![])),
                    _ => panic!("Can't add a non number at runtime"),
                }
            }
            Self::Sub(e1, e2, Some(a)) => {
                instructions.extend(e1.instructions()?);
                instructions.extend(e2.instructions()?);
                match a.expr_type {
                    ExprType::Int => instructions.push(Instr::Call("ins_52".to_string(), vec![])),
                    ExprType::Float => instructions.push(Instr::Call("ins_53".to_string(), vec![])),
                    _ => panic!("Can't subtract a non number"),
                }
            }
            Self::Mul(e1, e2, Some(a)) => {
                instructions.extend(e1.instructions()?);
                instructions.extend(e2.instructions()?);
                match a.expr_type {
                    ExprType::Int => instructions.push(Instr::Call("ins_54".to_string(), vec![])),
                    ExprType::Float => instructions.push(Instr::Call("ins_55".to_string(), vec![])),
                    _ => panic!("Can't subtract a non number"),
                }
            }
            Self::Div(e1, e2, Some(a)) => {
                instructions.extend(e1.instructions()?);
                instructions.extend(e2.instructions()?);
                match a.expr_type {
                    ExprType::Int => instructions.push(Instr::Call("ins_56".to_string(), vec![])),
                    ExprType::Float => instructions.push(Instr::Call("ins_57".to_string(), vec![])),
                    _ => panic!("Can't subtract a non number"),
                }
            }
            Self::Gt(e1, e2, Some(a)) => {
                instructions.extend(e1.instructions()?);
                instructions.extend(e2.instructions()?);
                match a.expr_type {
                    ExprType::Int => instructions.push(Instr::Call("ins_67".to_string(), vec![])),
                    ExprType::Float => instructions.push(Instr::Call("ins_68".to_string(), vec![])),
                    _ => panic!("Can't subtract a non number"),
                }
            }
            Self::Ge(e1, e2, Some(a)) => {
                instructions.extend(e1.instructions()?);
                instructions.extend(e2.instructions()?);
                match a.expr_type {
                    ExprType::Int => instructions.push(Instr::Call("ins_69".to_string(), vec![])),
                    ExprType::Float => instructions.push(Instr::Call("ins_70".to_string(), vec![])),
                    _ => panic!("Can't subtract a non number"),
                }
            }
            Self::Lt(e1, e2, Some(a)) => {
                instructions.extend(e1.instructions()?);
                instructions.extend(e2.instructions()?);
                match a.expr_type {
                    ExprType::Int => instructions.push(Instr::Call("ins_63".to_string(), vec![])),
                    ExprType::Float => instructions.push(Instr::Call("ins_64".to_string(), vec![])),
                    _ => panic!("Can't subtract a non number"),
                }
            }
            Self::Le(e1, e2, Some(a)) => {
                instructions.extend(e1.instructions()?);
                instructions.extend(e2.instructions()?);
                match a.expr_type {
                    ExprType::Int => instructions.push(Instr::Call("ins_65".to_string(), vec![])),
                    ExprType::Float => instructions.push(Instr::Call("ins_66".to_string(), vec![])),
                    _ => panic!("Can't subtract a non number"),
                }
            }
            Self::Eq(e1, e2, Some(a)) => {
                instructions.extend(e1.instructions()?);
                instructions.extend(e2.instructions()?);
                match a.expr_type {
                    ExprType::Int => instructions.push(Instr::Call("ins_59".to_string(), vec![])),
                    ExprType::Float => instructions.push(Instr::Call("ins_60".to_string(), vec![])),
                    _ => panic!("Can't subtract a non number"),
                }
            }
            Self::Ne(e1, e2, Some(a)) => {
                instructions.extend(e1.instructions()?);
                instructions.extend(e2.instructions()?);
                match a.expr_type {
                    ExprType::Int => instructions.push(Instr::Call("ins_61".to_string(), vec![])),
                    ExprType::Float => instructions.push(Instr::Call("ins_62".to_string(), vec![])),
                    _ => panic!("Can't subtract a non number"),
                }
            }
            Self::Modulo(e1, e2, Some(_)) => {
                instructions.extend(e1.instructions()?);
                instructions.extend(e2.instructions()?);
                instructions.push(Instr::Call("ins_58".to_string(), vec![]));
            }
            Self::BinAnd(e1, e2, Some(_)) => {
                instructions.extend(e1.instructions()?);
                instructions.extend(e2.instructions()?);
                instructions.push(Instr::Call("ins_77".to_string(), vec![]));
            }
            Self::Xor(e1, e2, Some(_)) => {
                instructions.extend(e1.instructions()?);
                instructions.extend(e2.instructions()?);
                instructions.push(Instr::Call("ins_75".to_string(), vec![]));
            }
            Self::BinOr(e1, e2, Some(_)) => {
                instructions.extend(e1.instructions()?);
                instructions.extend(e2.instructions()?);
                instructions.push(Instr::Call("ins_76".to_string(), vec![]));
            }
            Self::Or(e1, e2, Some(_)) => {
                instructions.extend(e1.instructions()?);
                instructions.extend(e2.instructions()?);
                instructions.push(Instr::Call("ins_73".to_string(), vec![]));
            }
            Self::And(e1, e2, Some(_)) => {
                instructions.extend(e1.instructions()?);
                instructions.extend(e2.instructions()?);
                instructions.push(Instr::Call("ins_74".to_string(), vec![]));
            }
            Self::Sin(e, Some(_)) => {
                instructions.extend(e.instructions()?);
                instructions.push(Instr::Call("ins_79".to_string(), vec![]));
            }
            Self::Cos(e, Some(_)) => {
                instructions.extend(e.instructions()?);
                instructions.push(Instr::Call("ins_80".to_string(), vec![]));
            }
            Self::Sqrt(e, Some(_)) => {
                instructions.extend(e.instructions()?);
                instructions.push(Instr::Call("ins_88".to_string(), vec![]));
            }
            Self::Id(i) => panic!("Unresolved identifier {i}"),
            Self::Str(_) => panic!("Can't push a string on the stack"),
            Self::Vararg(_) => panic!("Can't push a vararg on the stack"),
            _ => panic!(
                "Trying to generate instruction for non typed expression {:?}",
                self
            ),
        }
        Ok(instructions)
    }

    pub fn is_var(&self) -> bool {
        matches!(self, Self::VarInt(_) | Self::VarFloat(_))
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
            Self::Vararg(va) => {
                for v in va {
                    v.replace_id(id, to);
                }
            }
        }
    }

    pub fn replace_all_id(&mut self, ids: &std::collections::HashMap<String, Expr>) {
        for (id, ex) in ids {
            self.replace_id(id, ex);
        }
    }

    pub fn is_primitive(&self) -> bool {
        matches!(
            self,
            Self::Int(_)
                | Self::Float(_)
                | Self::Str(_)
                | Self::Id(_)
                | Self::VarFloat(_)
                | Self::VarInt(_)
        )
    }
}

fn resolve_expr(typ: &[String], args: &[AstNode]) -> Result<AstNode, Error> {
    use Expr as E;
    assert!(typ.len() == 1);
    let typ = &typ[0];
    Ok(AstNode::Expr(match &typ[..] {
        "Int" => {
            if args.len() != 1 {
                return Err(Error::Grammar("Expr::Int takes 1 param".to_owned()));
            }
            E::Int(
                args[0]
                    .clone()
                    .token_or(Error::Grammar("Expr::Int takes an int".to_owned()))?
                    .int_or(Error::Grammar("Expr::Int takes an int".to_owned()))?,
            )
        }
        "Float" => {
            if args.len() != 1 {
                return Err(Error::Grammar("Expr::Float takes 1 param".to_owned()));
            }
            E::Float(args[0].clone().token().float())
        }
        "Str" => {
            if args.len() != 1 {
                return Err(Error::Grammar("Expr::Str takes 1 param".to_owned()));
            }
            E::Str(args[0].clone().token().strn())
        }
        "Id" => {
            if args.len() != 1 {
                return Err(Error::Grammar("Expr::Id takes 2 param".to_owned()));
            }
            E::Id(args[0].clone().token().id())
        }
        "Add" => {
            if args.len() != 2 {
                return Err(Error::Grammar("Expr::Add takes 2 param".to_owned()));
            }
            E::Add(
                Box::new(args[0].clone().expr()),
                Box::new(args[1].clone().expr()),
                None,
            )
        }
        "Sub" => {
            if args.len() != 2 {
                return Err(Error::Grammar("Expr::Sub takes 2 param".to_owned()));
            }
            E::Sub(
                Box::new(args[0].clone().expr()),
                Box::new(args[1].clone().expr()),
                None,
            )
        }
        "Mul" => {
            if args.len() != 2 {
                return Err(Error::Grammar("Expr::Mul takes 2 param".to_owned()));
            }
            E::Mul(
                Box::new(args[0].clone().expr()),
                Box::new(args[1].clone().expr()),
                None,
            )
        }
        "Div" => {
            if args.len() != 2 {
                return Err(Error::Grammar("Expr::Div takes 2 param".to_owned()));
            }
            E::Div(
                Box::new(args[0].clone().expr()),
                Box::new(args[1].clone().expr()),
                None,
            )
        }
        "Mod" => {
            if args.len() != 2 {
                return Err(Error::Grammar("Expr::Mod takes 2 param".to_owned()));
            }
            E::Modulo(
                Box::new(args[0].clone().expr()),
                Box::new(args[1].clone().expr()),
                None,
            )
        }
        "Gt" => {
            if args.len() != 2 {
                return Err(Error::Grammar("Expr::Gt takes 2 param".to_owned()));
            }
            E::Gt(
                Box::new(args[0].clone().expr()),
                Box::new(args[1].clone().expr()),
                None,
            )
        }
        "Ge" => {
            if args.len() != 2 {
                return Err(Error::Grammar("Expr::Ge takes 2 param".to_owned()));
            }
            E::Ge(
                Box::new(args[0].clone().expr()),
                Box::new(args[1].clone().expr()),
                None,
            )
        }
        "Lt" => {
            if args.len() != 2 {
                return Err(Error::Grammar("Expr::Lt takes 2 param".to_owned()));
            }
            E::Lt(
                Box::new(args[0].clone().expr()),
                Box::new(args[1].clone().expr()),
                None,
            )
        }
        "Le" => {
            if args.len() != 2 {
                return Err(Error::Grammar("Expr::Le takes 2 param".to_owned()));
            }
            E::Le(
                Box::new(args[0].clone().expr()),
                Box::new(args[1].clone().expr()),
                None,
            )
        }
        "Eq" => {
            assert!(args.len() == 2);
            if args.len() != 2 {
                return Err(Error::Grammar("Expr::Ne takes 2 param".to_owned()));
            }
            E::Eq(
                Box::new(args[0].clone().expr()),
                Box::new(args[1].clone().expr()),
                None,
            )
        }
        "Ne" => {
            if args.len() != 2 {
                return Err(Error::Grammar("Expr::Ne takes 2 param".to_owned()));
            }
            E::Ne(
                Box::new(args[0].clone().expr()),
                Box::new(args[1].clone().expr()),
                None,
            )
        }
        "BinOr" => {
            if args.len() != 2 {
                return Err(Error::Grammar("Expr::BinOr takes 2 param".to_owned()));
            }
            E::BinOr(
                Box::new(args[0].clone().expr()),
                Box::new(args[1].clone().expr()),
                None,
            )
        }
        "BinAnd" => {
            if args.len() != 2 {
                return Err(Error::Grammar("Expr::BinAnd takes 2 param".to_owned()));
            }
            E::BinAnd(
                Box::new(args[0].clone().expr()),
                Box::new(args[1].clone().expr()),
                None,
            )
        }
        "Xor" => {
            if args.len() != 2 {
                return Err(Error::Grammar("Expr::Xor takes 2 param".to_owned()));
            }
            E::Xor(
                Box::new(args[0].clone().expr()),
                Box::new(args[1].clone().expr()),
                None,
            )
        }
        "Or" => {
            if args.len() != 2 {
                return Err(Error::Grammar("Expr::Or takes 2 param".to_owned()));
            }
            E::Or(
                Box::new(args[0].clone().expr()),
                Box::new(args[1].clone().expr()),
                None,
            )
        }
        "And" => {
            if args.len() != 2 {
                return Err(Error::Grammar("Expr::And takes 2 param".to_owned()));
            }
            E::And(
                Box::new(args[0].clone().expr()),
                Box::new(args[1].clone().expr()),
                None,
            )
        }
        "Uminus" => {
            if args.len() != 1 {
                return Err(Error::Grammar("Expr::Uminus takes 1 param".to_owned()));
            }
            E::Uminus(Box::new(args[0].clone().expr()), None)
        }
        "Not" => {
            if args.len() != 1 {
                return Err(Error::Grammar("Expr::Not takes 1 param".to_owned()));
            }
            E::Not(Box::new(args[0].clone().expr()), None)
        }
        "Sin" => {
            if args.len() != 1 {
                return Err(Error::Grammar("Expr::Sin takes 1 param".to_owned()));
            }
            E::Sin(Box::new(args[0].clone().expr()), None)
        }
        "Cos" => {
            if args.len() != 1 {
                return Err(Error::Grammar("Expr::Cos takes 1 param".to_owned()));
            }
            E::Cos(Box::new(args[0].clone().expr()), None)
        }
        "Sqrt" => {
            if args.len() != 1 {
                return Err(Error::Grammar("Expr::Sqrt takes 1 param".to_owned()));
            }
            E::Sqrt(Box::new(args[0].clone().expr()), None)
        }
        "Var" => {
            if args.len() != 1 {
                return Err(Error::Grammar("Expr::Var takes 1 param".to_owned()));
            }
            let AstNode::Data{ref dtype, ref children} = args[0] else { return Err(Error::ShouldNeverBeThere) };
            match &dtype[..] {
                "VarExpr::Int" => {
                    if children.len() != 1 {
                        return Err(Error::ShouldNeverBeThere);
                    }
                    E::VarInt(children[0].clone().token().int_or(Error::Grammar(
                        "Arg to VarExpr::Int should be an int".to_owned(),
                    ))?)
                }
                "VarExpr::Float" => {
                    if children.len() != 1 {
                        return Err(Error::ShouldNeverBeThere);
                    }
                    E::VarFloat(children[0].clone().token().float_or(Error::Grammar(
                        "Arg to VarExpr::Float should be a float".to_owned(),
                    ))?)
                }
                _ => {
                    return Err(Error::ShouldNeverBeThere);
                }
            }
        }
        t => {
            return Err(Error::Grammar(format!("unknown type {t} for Expr")));
        }
    }))
}

fn resolve_varexpr(typ: &[String], args: &[AstNode]) -> Result<AstNode, Error> {
    if typ.len() != 1 {
        return Err(Error::Grammar(
            "VarExpr command is composed of 1 subcommand".to_owned(),
        ));
    }
    let typ = &typ[0];
    Ok(match &typ[..] {
        "Int" => {
            if args.len() != 1 {
                return Err(Error::Grammar(
                    "VarExpr::Int subcommand takes 1 param".to_owned(),
                ));
            }
            AstNode::Data {
                dtype: "VarExpr::Int".to_string(),
                children: args.to_vec(),
            }
        }
        "Float" => {
            if args.len() != 1 {
                return Err(Error::Grammar(
                    "VarExpr::Float subcommand takes 1 param".to_owned(),
                ));
            }
            AstNode::Data {
                dtype: "VarExpr::Float".to_string(),
                children: args.to_vec(),
            }
        }
        "MInt" => {
            if args.len() != 1 {
                return Err(Error::Grammar(
                    "VarExpr::MInt subcommand takes 1 param".to_owned(),
                ));
            }
            let int = -args[0].clone().token().int_or(Error::Grammar(
                "Param to MInt should be an integer".to_owned(),
            ))?;
            let int = AstNode::Token(Token::Int(int));
            AstNode::Data {
                dtype: "VarExpr::Int".to_string(),
                children: vec![int],
            }
        }
        "MFloat" => {
            if args.len() != 1 {
                return Err(Error::Grammar(
                    "VarExpr::MFloat subcommand takes 1 param".to_owned(),
                ));
            }
            let float = -args[0].clone().token().float_or(Error::Grammar(
                "Param to MInt should be an float".to_owned(),
            ))?;
            let float = AstNode::Token(Token::Float(float));
            AstNode::Data {
                dtype: "VarExpr::Float".to_string(),
                children: vec![float],
            }
        }
        t => {
            return Err(Error::Grammar(format!("unknown type {t} for VarExpr")));
        }
    })
}

pub fn fill_executor(resolver: &mut AstResolver<AstNode>) {
    resolver.add_func("Expr", resolve_expr);
    resolver.add_func("VarExpr", resolve_varexpr);
}
