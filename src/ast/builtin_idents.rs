use std::collections::HashMap;

use super::*;

pub fn replace(instrs: &Vec<Instr>) -> Vec<Instr> {
    let mut builtin_idents = HashMap::new();
    builtin_idents.insert("true".to_owned(), Expr::Int(1));
    builtin_idents.insert("false".to_owned(), Expr::Int(0));
    builtin_idents.insert("NULL".to_owned(), Expr::Int(-999999));
    builtin_idents.insert("NULLF".to_owned(), Expr::Float(-999999.));
    builtin_idents.insert("PI".to_owned(), Expr::Float(3.1415926535));

    let mut new_instrs = Vec::new();
    for i in instrs {
        match i {
            Instr::Call(name, exprs) => {
                let mut vars = Vec::new();
                for e in exprs {
                    let mut e = e.clone();
                    e.replace_all_id(&builtin_idents);
                    vars.push(e);
                }
                new_instrs.push(Instr::Call(name.clone(), vars));
            }
            Instr::PushExpr(expr) => {
                let mut e = expr.clone();
                e.replace_all_id(&builtin_idents);
                new_instrs.push(Instr::PushExpr(e));
            }
            Instr::Bloc(l) => new_instrs.push(Instr::Bloc(replace(l))),
            Instr::Label(l) => {
                if builtin_idents.contains_key(l) {
                    panic!("Can't use identifier {l} as a label");
                }
                new_instrs.push(i.clone());
            }
            Instr::Loop(l) => new_instrs.push(Instr::Loop(replace(l))),
            Instr::While(e, l) => {
                let mut e = e.clone();
                e.replace_all_id(&builtin_idents);
                new_instrs.push(Instr::While(e, replace(l)));
            }
            Instr::DoWhile(e, l) => {
                let mut e = e.clone();
                e.replace_all_id(&builtin_idents);
                new_instrs.push(Instr::DoWhile(e, replace(l)));
            }
            Instr::If(e, l1, l2) => {
                let mut e = e.clone();
                e.replace_all_id(&builtin_idents);
                new_instrs.push(Instr::If(e, replace(l1), replace(l2)));
            }
            _ => new_instrs.push(i.clone()),
        }
    }
    new_instrs
}
