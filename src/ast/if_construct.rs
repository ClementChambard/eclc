use crate::error::{report_error_ext, Error};

use super::*;

pub fn desugar_bloc(
    sub: &Sub,
    bloc: &Vec<Instr>,
    lbl_seed: &mut usize,
) -> Result<Vec<Instr>, Error> {
    let mut new_instructions = Vec::new();
    for i in bloc {
        match i {
            Instr::If(cond, if_bloc, else_bloc) => {
                let if_code = desugar(sub, cond, if_bloc, else_bloc, lbl_seed)?;
                new_instructions.extend(if_code);
            }
            Instr::Bloc(l) => {
                new_instructions.extend(desugar_bloc(sub, l, lbl_seed)?);
            }
            Instr::Loop(l) => {
                let loop_code = desugar_bloc(sub, l, lbl_seed)?;
                new_instructions.push(Instr::Loop(loop_code));
            }
            Instr::While(e, l) => {
                let loop_code = desugar_bloc(sub, l, lbl_seed)?;
                new_instructions.push(Instr::While(e.clone(), loop_code));
            }
            Instr::DoWhile(e, l) => {
                let loop_code = desugar_bloc(sub, l, lbl_seed)?;
                new_instructions.push(Instr::DoWhile(e.clone(), loop_code));
            }
            _ => new_instructions.push(i.clone()),
        }
    }
    Ok(new_instructions)
}

pub fn desugar(
    sub: &Sub,
    cond: &Expr,
    if_bloc: &Vec<Instr>,
    else_bloc: &Vec<Instr>,
    lbl_seed: &mut usize,
) -> Result<Vec<Instr>, Error> {
    let if_bloc = desugar_bloc(sub, if_bloc, lbl_seed)?;
    let else_bloc = desugar_bloc(sub, else_bloc, lbl_seed)?;
    let else_bloc_empty = else_bloc.is_empty();
    let mut new_instructions = Vec::new();
    let mut e = cond.clone();
    e.anotate()?;
    e.constant_fold();
    if e.get_type()? != ExprType::Int {
        report_error_ext(
            &e.loc(),
            "Condition expression should be of type int",
            "This expression should have type int",
        );
        return Err(Error::Simple("If condition must be Int".to_owned()));
    }
    if e.is_primitive() && !e.is_var() {
        let i = e.int();
        if *i.val() != 0 {
            new_instructions.extend(if_bloc);
        } else {
            new_instructions.extend(else_bloc);
        }
        return Ok(new_instructions);
    }
    new_instructions.push(Instr::PushExpr(cond.clone()));
    let else_label = sub.gen_label(lbl_seed);
    new_instructions.push(Instr::Call(
        "ins_14".to_string().into(),
        vec![
            Expr::Id(else_label.clone().into()),
            Expr::Float(0.0.into()), // TODO: should not be 0 but the time of the if
        ],
    ));
    new_instructions.extend(if_bloc);
    if else_bloc_empty {
        new_instructions.push(Instr::Label(else_label.into()));
    } else {
        let endif_label = sub.gen_label(lbl_seed);
        new_instructions.push(Instr::Call(
            "ins_12".to_string().into(),
            vec![
                Expr::Id(endif_label.clone().into()),
                Expr::Float(0.0.into()), // TODO: should be the time of last if_bloc instruction
            ],
        ));
        new_instructions.push(Instr::Label(else_label.into()));
        new_instructions.extend(else_bloc);
        new_instructions.push(Instr::Label(endif_label.into()));
    }
    Ok(new_instructions)
}
