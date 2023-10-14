use crate::error::Error;

use super::*;

lazy_static! {
    static ref BUILTIN_IDENTS: std::collections::HashMap<String, Expr> = {
        let mut builtin_idents = std::collections::HashMap::new();
        builtin_idents.insert("true".to_owned(), Expr::Int(1.into()));
        builtin_idents.insert("false".to_owned(), Expr::Int(0.into()));
        builtin_idents.insert("NULL".to_owned(), Expr::Int(Located::from(-999999)));
        builtin_idents.insert("NULLF".to_owned(), Expr::Float(Located::from(-999999.0)));
        builtin_idents.insert("PI".to_owned(), Expr::Float(std::f32::consts::PI.into()));

        // Et ex
        builtin_idents.insert("EX_SPEEDUP".to_owned(), Expr::Int(1.into())); // Note: formerly EX_DIST
        builtin_idents.insert("EX_ANIM".to_owned(), Expr::Int(2.into()));
        builtin_idents.insert("EX_ACCEL".to_owned(), Expr::Int(4.into()));
        builtin_idents.insert("EX_ANGLE_ACCEL".to_owned(), Expr::Int(8.into()));
        builtin_idents.insert("EX_STEP".to_owned(), Expr::Int(16.into())); // Note: formerly EX_ANGLE
        builtin_idents.insert("EX_BOUNCE".to_owned(), Expr::Int(64.into()));
        builtin_idents.insert("EX_INVULN".to_owned(), Expr::Int(128.into()));
        builtin_idents.insert("EX_OFFSCREEN".to_owned(), Expr::Int(256.into()));
        builtin_idents.insert("EX_SETSPRITE".to_owned(), Expr::Int(512.into()));
        builtin_idents.insert("EX_DELETE".to_owned(), Expr::Int(1024.into()));
        builtin_idents.insert("EX_PLAYSOUND".to_owned(), Expr::Int(2048.into()));
        builtin_idents.insert("EX_WRAP".to_owned(), Expr::Int(4096.into()));
        builtin_idents.insert("EX_SHOOTPREP".to_owned(), Expr::Int(8192.into()));
        builtin_idents.insert("EX_SHOOT".to_owned(), Expr::Int(16384.into()));
        builtin_idents.insert("EX_REACT".to_owned(), Expr::Int(32768.into()));
        builtin_idents.insert("EX_GOTO".to_owned(), Expr::Int(65536.into()));
        builtin_idents.insert("EX_MOVE".to_owned(), Expr::Int(131072.into()));
        builtin_idents.insert("EX_VEL".to_owned(), Expr::Int(262144.into()));
        builtin_idents.insert("EX_VELADD".to_owned(), Expr::Int(524288.into()));
        builtin_idents.insert("EX_BRIGHT".to_owned(), Expr::Int(1048576.into()));
        builtin_idents.insert("EX_ACCELWEIRD".to_owned(), Expr::Int(2097152.into())); // Note: formerly EX_VELTIME
        builtin_idents.insert("EX_SIZE".to_owned(), Expr::Int(4194304.into()));
        builtin_idents.insert("EX_SAVE".to_owned(), Expr::Int(8388608.into())); // Note: formerly EX_SAVEANGLE
        builtin_idents.insert("EX_ENMCREATE".to_owned(), Expr::Int(16777216.into())); // Note: formerly EX_SPECIAL
        builtin_idents.insert("EX_LAYER".to_owned(), Expr::Int(33554432.into()));
        builtin_idents.insert("EX_DELAY".to_owned(), Expr::Int(67108864.into()));
        builtin_idents.insert("EX_LASER".to_owned(), Expr::Int(134217728.into()));
        builtin_idents.insert("EX_HITBOX".to_owned(), Expr::Int(536870912.into()));
        builtin_idents.insert("EX_WAIT".to_owned(), Expr::Int(Located::from(-2147483648)));

        /* aim modes */
        /* AT = aim at player, ST = static aim */
        builtin_idents.insert("AIM_AT".to_owned(), Expr::Int(0.into()));
        builtin_idents.insert("AIM_ST".to_owned(), Expr::Int(1.into()));
        builtin_idents.insert("AIM_AT_RING".to_owned(), Expr::Int(2.into()));
        builtin_idents.insert("AIM_ST_RING".to_owned(), Expr::Int(3.into()));
        builtin_idents.insert("AIM_AWAY_RING".to_owned(), Expr::Int(4.into()));
        builtin_idents.insert("AIM_ST_RING2".to_owned(), Expr::Int(5.into()));
        builtin_idents.insert("AIM_RAND".to_owned(), Expr::Int(6.into()));
        builtin_idents.insert("AIM_RAND_RING".to_owned(), Expr::Int(7.into()));
        builtin_idents.insert("AIM_MEEK".to_owned(), Expr::Int(8.into()));
        builtin_idents.insert("AIM_AT_PYRAMID".to_owned(), Expr::Int(9.into()));
        builtin_idents.insert("AIM_ST_PYRAMID".to_owned(), Expr::Int(10.into()));
        builtin_idents.insert("AIM_PEANUT".to_owned(), Expr::Int(11.into()));
        builtin_idents.insert("AIM_PEANUT2".to_owned(), Expr::Int(12.into()));
        builtin_idents
    };
}

pub fn replace(instrs: &Vec<Instr>) -> Result<Vec<Instr>, Error> {
    let mut new_instrs = Vec::new();
    for i in instrs {
        match i {
            Instr::Call(name, exprs) => {
                let mut vars = Vec::new();
                for e in exprs {
                    let mut e = e.clone();
                    e.replace_all_id(&BUILTIN_IDENTS);
                    vars.push(e);
                }
                new_instrs.push(Instr::Call(name.clone(), vars));
            }
            Instr::PushExpr(expr) => {
                let mut e = expr.clone();
                e.replace_all_id(&BUILTIN_IDENTS);
                new_instrs.push(Instr::PushExpr(e));
            }
            Instr::Bloc(l) => new_instrs.push(Instr::Bloc(replace(l)?)),
            Instr::Label(l) => {
                if BUILTIN_IDENTS.contains_key(l.val()) {
                    return Err(Error::Simple(format!(
                        "Can't use identifier {} as a label",
                        l.val()
                    )));
                }
                new_instrs.push(i.clone());
            }
            Instr::Loop(l) => new_instrs.push(Instr::Loop(replace(l)?)),
            Instr::While(e, l) => {
                let mut e = e.clone();
                e.replace_all_id(&BUILTIN_IDENTS);
                new_instrs.push(Instr::While(e, replace(l)?));
            }
            Instr::DoWhile(e, l) => {
                let mut e = e.clone();
                e.replace_all_id(&BUILTIN_IDENTS);
                new_instrs.push(Instr::DoWhile(e, replace(l)?));
            }
            Instr::If(e, l1, l2) => {
                let mut e = e.clone();
                e.replace_all_id(&BUILTIN_IDENTS);
                new_instrs.push(Instr::If(e, replace(l1)?, replace(l2)?));
            }
            Instr::TimeLabel(_, _)
            | Instr::RankLabel(_)
            | Instr::Break
            | Instr::Continue
            | Instr::Affect(_, _)
            | Instr::VarFloat(_, _)
            | Instr::VarInt(_, _) => new_instrs.push(i.clone()),
        }
    }
    Ok(new_instrs)
}
