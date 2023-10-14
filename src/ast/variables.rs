use crate::error::Error;

use super::*;

#[derive(Clone)]
pub enum Variable {
    Int(i32, String),
    Float(f32, String),
}

impl Variable {
    pub fn name(&self) -> &str {
        match self {
            Self::Int(_, s) => s,
            Self::Float(_, s) => s,
        }
    }

    pub fn pop_instr(&self) -> Instr {
        let ins_name = match self {
            Self::Int(_, _) => "ins_43".to_owned(),
            Self::Float(_, _) => "ins_45".to_owned(),
        };
        let e = self.expr();
        Instr::Call(ins_name.into(), vec![e])
    }

    pub fn expr(&self) -> Expr {
        match self {
            Self::Int(i, _) => Expr::VarInt((*i).into()),
            Self::Float(i, _) => Expr::VarFloat((*i).into()),
        }
    }
}

#[derive(Clone)]
pub struct Scope {
    variables: Vec<Variable>,
    parent_scope: Vec<Scope>,
    local_max_offset: i32,
    pub max_offset: i32,
}

impl Scope {
    pub fn new() -> Self {
        Self {
            variables: vec![],
            parent_scope: vec![],
            local_max_offset: 0,
            max_offset: 0,
        }
    }

    pub fn push_scope(&self) -> Self {
        Self {
            variables: vec![],
            max_offset: self.max_offset,
            local_max_offset: self.local_max_offset,
            parent_scope: vec![self.clone()],
        }
    }

    pub fn pop_scope(mut self) -> Result<Self, Error> {
        assert!(!self.parent_scope.is_empty());
        let mut s = self
            .parent_scope
            .pop()
            .ok_or(Error::BackEnd("Scope has no parent".to_owned()))?;
        s.max_offset = self.max_offset;
        Ok(s)
    }

    pub fn add_var(&mut self, v: &str, int_1_float_2: i8) -> Result<(), Error> {
        let o = self.local_max_offset;
        self.local_max_offset += 4;
        if self.local_max_offset > self.max_offset {
            self.max_offset = self.local_max_offset;
        }
        let var = if int_1_float_2 == 1 {
            Variable::Int(o, v.to_string())
        } else {
            Variable::Float(o as f32, v.to_string())
        };
        if self.variables.iter().any(|va| va.name() == v) {
            return Err(Error::Simple(format!("Variable {v} already exists")));
        }
        self.variables.push(var);
        Ok(())
    }

    pub fn assign(&self, v: &str, expr: &Expr) -> Result<Vec<Instr>, Error> {
        let var = self.get_var(v).ok_or(Error::Simple(format!(
            "Variable {v} doesn't exist for assignment"
        )))?;
        Ok(vec![Instr::PushExpr(expr.clone()), var.pop_instr()])
    }

    pub fn get_var<'a>(&'a self, name: &str) -> Option<&'a Variable> {
        let found = self.variables.iter().find(|v| v.name() == name);
        match found {
            None => {
                if self.parent_scope.is_empty() {
                    return None;
                }
                self.parent_scope[0].get_var(name)
            }
            Some(_) => found,
        }
    }
}

pub fn replace_in_expr(scope: &Scope, e: &mut Expr) {
    match e {
        Expr::VarInt(_) | Expr::VarFloat(_) | Expr::Int(_) | Expr::Float(_) | Expr::Str(_) => {}
        Expr::Vararg(ref mut va) => {
            for v in va {
                replace_in_expr(scope, v);
            }
        }
        Expr::Add(ref mut e1, ref mut e2, _)
        | Expr::Sub(ref mut e1, ref mut e2, _)
        | Expr::Mul(ref mut e1, ref mut e2, _)
        | Expr::Div(ref mut e1, ref mut e2, _)
        | Expr::Modulo(ref mut e1, ref mut e2, _)
        | Expr::Ge(ref mut e1, ref mut e2, _)
        | Expr::Lt(ref mut e1, ref mut e2, _)
        | Expr::Le(ref mut e1, ref mut e2, _)
        | Expr::Gt(ref mut e1, ref mut e2, _)
        | Expr::Eq(ref mut e1, ref mut e2, _)
        | Expr::Ne(ref mut e1, ref mut e2, _)
        | Expr::Xor(ref mut e1, ref mut e2, _)
        | Expr::BinOr(ref mut e1, ref mut e2, _)
        | Expr::BinAnd(ref mut e1, ref mut e2, _)
        | Expr::Or(ref mut e1, ref mut e2, _)
        | Expr::And(ref mut e1, ref mut e2, _) => {
            replace_in_expr(scope, e1);
            replace_in_expr(scope, e2);
        }
        Expr::Sin(ref mut e, _)
        | Expr::Cos(ref mut e, _)
        | Expr::Sqrt(ref mut e, _)
        | Expr::Not(ref mut e, _)
        | Expr::Uminus(ref mut e, _) => replace_in_expr(scope, e),
        Expr::Id(s) => {
            if let Some(v) = scope.get_var(s.val()) {
                *e = v.expr();
            }
        }
    }
}

pub fn replace_in_bloc(scope: &mut Scope, ins: &Vec<Instr>) -> Result<Vec<Instr>, Error> {
    let mut new_ins = Vec::new();
    for i in ins {
        match i {
            Instr::Break
            | Instr::Continue
            | Instr::TimeLabel(_, _)
            | Instr::Label(_)
            | Instr::RankLabel(_) => new_ins.push(i.clone()),
            Instr::Call(ins_name, exprs) => {
                let mut new_exprs = exprs.clone();
                for e in &mut new_exprs {
                    replace_in_expr(scope, e);
                }
                // check for vars in exprs
                new_ins.push(Instr::Call(ins_name.clone(), new_exprs));
            }
            Instr::PushExpr(e) => {
                let mut new_expr = e.clone();
                replace_in_expr(scope, &mut new_expr);
                new_ins.push(Instr::PushExpr(new_expr));
            }
            Instr::Bloc(l) => {
                let mut new_scope = scope.push_scope();
                let new_l = replace_in_bloc(&mut new_scope, l)?;
                *scope = new_scope.pop_scope()?;
                new_ins.extend(new_l);
            }
            Instr::Loop(l) => {
                let mut new_scope = scope.push_scope();
                let new_l = replace_in_bloc(&mut new_scope, l)?;
                *scope = new_scope.pop_scope()?;
                new_ins.push(Instr::Loop(new_l));
            }
            Instr::If(e, l1, l2) => {
                let mut new_e = e.clone();
                replace_in_expr(scope, &mut new_e);
                let mut new_scope = scope.push_scope();
                let new_l1 = replace_in_bloc(&mut new_scope, l1)?;
                *scope = new_scope.pop_scope()?;
                let mut new_scope = scope.push_scope();
                let new_l2 = replace_in_bloc(&mut new_scope, l2)?;
                *scope = new_scope.pop_scope()?;
                new_ins.push(Instr::If(new_e, new_l1, new_l2));
            }
            Instr::While(e, l) => {
                let mut new_e = e.clone();
                replace_in_expr(scope, &mut new_e);
                let mut new_scope = scope.push_scope();
                let new_l = replace_in_bloc(&mut new_scope, l)?;
                *scope = new_scope.pop_scope()?;
                new_ins.push(Instr::While(new_e, new_l));
            }
            Instr::DoWhile(e, l) => {
                let mut new_e = e.clone();
                replace_in_expr(scope, &mut new_e);
                let mut new_scope = scope.push_scope();
                let new_l = replace_in_bloc(&mut new_scope, l)?;
                *scope = new_scope.pop_scope()?;
                new_ins.push(Instr::DoWhile(new_e, new_l));
            }
            Instr::Affect(v, e) => {
                let mut new_e = e.clone();
                replace_in_expr(scope, &mut new_e);
                new_ins.extend(scope.assign(v.val(), &new_e)?);
            }
            Instr::VarInt(v, e_opt) => {
                scope.add_var(v.val(), 1)?;
                match e_opt {
                    Some(e) => {
                        let mut new_e = e.clone();
                        replace_in_expr(scope, &mut new_e);
                        new_ins.extend(scope.assign(v.val(), &new_e)?);
                    }
                    None => {}
                }
            }
            Instr::VarFloat(v, e_opt) => {
                scope.add_var(v.val(), 2)?;
                match e_opt {
                    Some(e) => {
                        let mut new_e = e.clone();
                        replace_in_expr(scope, &mut new_e);
                        new_ins.extend(scope.assign(v.val(), &new_e)?);
                    }
                    None => {}
                }
            }
        }
    }
    Ok(new_ins)
}
