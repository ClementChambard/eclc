use super::*;

pub fn desugar_bloc(sub: &Sub, bloc: &Vec<Instr>, lbl_seed: &mut usize) -> Vec<Instr> {
    let mut new_instr = Vec::new();
    for i in bloc {
        match i {
            Instr::Loop(l) => new_instr.extend(desugar(sub, l, lbl_seed)),
            Instr::While(e, l) => {
                let loop_code = desugar_bloc(sub, l, lbl_seed);
                new_instr.push(Instr::While(e.clone(), loop_code));
            }
            Instr::DoWhile(e, l) => {
                let loop_code = desugar_bloc(sub, l, lbl_seed);
                new_instr.push(Instr::DoWhile(e.clone(), loop_code));
            }
            _ => new_instr.push(i.clone()),
        }
    }
    new_instr
}

pub fn desugar(sub: &Sub, bloc: &Vec<Instr>, lbl_seed: &mut usize) -> Vec<Instr> {
    let mut instructions = Vec::new();
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
                    "ins_12".to_string(),
                    vec![Expr::Id(break_label.clone()), Expr::Int(0)], // TODO: int is the time
                ));
            }
            Instr::Continue => bloc_instructions.push(Instr::Call(
                "ins_12".to_string(),
                vec![Expr::Id(loop_label.clone()), Expr::Int(0)], // TODO: int is the time
            )),
            _ => bloc_instructions.push(i.clone()),
        }
    }
    let bloc = desugar_bloc(sub, &bloc_instructions, lbl_seed);
    instructions.push(Instr::Label(loop_label.clone()));
    instructions.extend(bloc);
    instructions.push(Instr::Call(
        "ins_12".to_string(),
        vec![Expr::Id(loop_label), Expr::Int(0)], // TODO: int is the time
    ));
    if has_break {
        instructions.push(Instr::Label(break_label));
    }
    instructions
}
