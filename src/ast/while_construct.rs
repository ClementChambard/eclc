use crate::error::Error;

use super::*;

pub fn desugar_bloc(
    sub: &Sub,
    bloc: &Vec<Instr>,
    lbl_seed: &mut usize,
) -> Result<Vec<Instr>, Error> {
    let mut new_instr = Vec::new();
    for i in bloc {
        match i {
            Instr::While(e, l) => new_instr.extend(desugar(sub, e, l, lbl_seed, true)?),
            Instr::DoWhile(e, l) => new_instr.extend(desugar(sub, e, l, lbl_seed, false)?),
            _ => new_instr.push(i.clone()),
        }
    }
    Ok(new_instr)
}

pub fn desugar(
    sub: &Sub,
    cond: &Expr,
    bloc: &Vec<Instr>,
    lbl_seed: &mut usize,
    first_jump: bool,
) -> Result<Vec<Instr>, Error> {
    let mut e = cond.clone();
    e.anotate()?;
    e.constant_fold();
    if e.get_type()? != ExprType::Int {
        return Err(Error::Simple(
            "Condition for while should be type Int".to_owned(),
        ));
    }
    if e.is_primitive() && !e.is_var() {
        let i = e.int().into_val();
        if i != 0 {
            return Ok(super::loop_construct::desugar(sub, bloc, lbl_seed));
        } else if first_jump {
            return Ok(vec![]);
        } else {
            return Ok(bloc.clone());
        }
    }
    let mut instructions = Vec::new();
    let first_jump_lbl = if first_jump {
        let lbl = sub.gen_label(lbl_seed);
        instructions.push(Instr::Call(
            "ins_12".to_string().into(),
            vec![Expr::Id(lbl.clone().into()), Expr::Float(0.0.into())], // TODO: int is the time
        ));
        lbl
    } else {
        String::new()
    };

    let loop_label = sub.gen_label(lbl_seed);
    let mut break_label = String::new();
    let mut has_break = false;
    let mut bloc_instructions = Vec::new();
    for i in bloc {
        match i {
            Instr::Break => {
                if !has_break {
                    has_break = true;
                    break_label = sub.gen_label(lbl_seed);
                }
                bloc_instructions.push(Instr::Call(
                    "ins_12".to_string().into(),
                    vec![
                        Expr::Id(break_label.clone().into()),
                        Expr::Float(0.0.into()),
                    ], // TODO: int is the time
                ));
            }
            Instr::Continue => bloc_instructions.push(Instr::Call(
                "ins_12".to_string().into(),
                vec![Expr::Id(loop_label.clone().into()), Expr::Float(0.0.into())], // TODO: int is the time
            )),
            _ => bloc_instructions.push(i.clone()),
        }
    }
    let bloc = desugar_bloc(sub, &bloc_instructions, lbl_seed)?;
    instructions.push(Instr::Label(loop_label.clone().into()));
    instructions.extend(bloc);
    if first_jump {
        instructions.push(Instr::Label(first_jump_lbl.into()));
    }
    instructions.push(Instr::PushExpr(e));
    instructions.push(Instr::Call(
        "ins_14".to_string().into(),
        vec![Expr::Id(loop_label.into()), Expr::Float(0.0.into())], // TODO: int is the time
    ));
    if has_break {
        instructions.push(Instr::Label(break_label.into()));
    }
    Ok(instructions)
}
