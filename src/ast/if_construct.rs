use super::*;

pub fn desugar_bloc(sub: &Sub, bloc: &Vec<Instr>, lbl_seed: &mut usize) -> Vec<Instr> {
    let mut new_instructions = Vec::new();
    for i in bloc {
        match i {
            Instr::If(cond, if_bloc, else_bloc) => {
                let if_code = desugar(sub, cond, if_bloc, else_bloc, lbl_seed);
                new_instructions.extend(if_code);
            }
            Instr::Bloc(l) => {
                new_instructions.extend(desugar_bloc(sub, l, lbl_seed));
            }
            Instr::Loop(l) => {
                let loop_code = desugar_bloc(sub, l, lbl_seed);
                new_instructions.push(Instr::Loop(loop_code));
            }
            Instr::While(e, l) => {
                let loop_code = desugar_bloc(sub, l, lbl_seed);
                new_instructions.push(Instr::While(e.clone(), loop_code));
            }
            Instr::DoWhile(e, l) => {
                let loop_code = desugar_bloc(sub, l, lbl_seed);
                new_instructions.push(Instr::DoWhile(e.clone(), loop_code));
            }
            _ => new_instructions.push(i.clone()),
        }
    }
    new_instructions
}

pub fn desugar(
    sub: &Sub,
    cond: &Expr,
    if_bloc: &Vec<Instr>,
    else_bloc: &Vec<Instr>,
    lbl_seed: &mut usize,
) -> Vec<Instr> {
    let if_bloc = desugar_bloc(sub, if_bloc, lbl_seed);
    let else_bloc = desugar_bloc(sub, else_bloc, lbl_seed);
    let else_bloc_empty = else_bloc.is_empty();
    let mut new_instructions = Vec::new();
    let mut e = cond.clone();
    e.anotate();
    e.constant_fold();
    if e.get_type() != ExprType::Int {
        panic!("If condition must be Int");
    }
    if e.is_primitive() && !e.is_var() {
        let i = e.int();
        if i != 0 {
            new_instructions.extend(if_bloc);
        } else {
            new_instructions.extend(else_bloc);
        }
        return new_instructions;
    }
    new_instructions.push(Instr::PushExpr(cond.clone()));
    let else_label = sub.gen_label(lbl_seed);
    new_instructions.push(Instr::Call(
        "ins_14".to_string(),
        vec![
            Expr::Id(else_label.clone()),
            Expr::Float(0.), // TODO: should not be 0 but the time of the if
        ],
    ));
    new_instructions.extend(if_bloc);
    if else_bloc_empty {
        new_instructions.push(Instr::Label(else_label));
    } else {
        let endif_label = sub.gen_label(lbl_seed);
        new_instructions.push(Instr::Call(
            "ins_12".to_string(),
            vec![
                Expr::Id(endif_label.clone()),
                Expr::Float(0.), // TODO: should be the time of last if_bloc instruction
            ],
        ));
        new_instructions.push(Instr::Label(else_label));
        new_instructions.extend(else_bloc);
        new_instructions.push(Instr::Label(endif_label));
    }
    new_instructions
}
