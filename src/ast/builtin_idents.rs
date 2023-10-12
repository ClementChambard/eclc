use crate::error::Error;

use super::*;

lazy_static! {
    static ref BUILTIN_IDENTS: std::collections::HashMap<String, Expr> = {
        let mut builtin_idents = std::collections::HashMap::new();
        builtin_idents.insert("true".to_owned(), Expr::Int(1));
        builtin_idents.insert("false".to_owned(), Expr::Int(0));
        builtin_idents.insert("NULL".to_owned(), Expr::Int(-999999));
        builtin_idents.insert("NULLF".to_owned(), Expr::Float(-999999.));
        builtin_idents.insert("PI".to_owned(), Expr::Float(std::f32::consts::PI));

        // Et ex
        builtin_idents.insert("EX_SPEEDUP".to_owned(), Expr::Int(1)); // Note: formerly EX_DIST
        builtin_idents.insert("EX_ANIM".to_owned(), Expr::Int(2));
        builtin_idents.insert("EX_ACCEL".to_owned(), Expr::Int(4));
        builtin_idents.insert("EX_ANGLE_ACCEL".to_owned(), Expr::Int(8));
        builtin_idents.insert("EX_STEP".to_owned(), Expr::Int(16)); // Note: formerly EX_ANGLE
        builtin_idents.insert("EX_BOUNCE".to_owned(), Expr::Int(64));
        builtin_idents.insert("EX_INVULN".to_owned(), Expr::Int(128));
        builtin_idents.insert("EX_OFFSCREEN".to_owned(), Expr::Int(256));
        builtin_idents.insert("EX_SETSPRITE".to_owned(), Expr::Int(512));
        builtin_idents.insert("EX_DELETE".to_owned(), Expr::Int(1024));
        builtin_idents.insert("EX_PLAYSOUND".to_owned(), Expr::Int(2048));
        builtin_idents.insert("EX_WRAP".to_owned(), Expr::Int(4096));
        builtin_idents.insert("EX_SHOOTPREP".to_owned(), Expr::Int(8192));
        builtin_idents.insert("EX_SHOOT".to_owned(), Expr::Int(16384));
        builtin_idents.insert("EX_REACT".to_owned(), Expr::Int(32768));
        builtin_idents.insert("EX_GOTO".to_owned(), Expr::Int(65536));
        builtin_idents.insert("EX_MOVE".to_owned(), Expr::Int(131072));
        builtin_idents.insert("EX_VEL".to_owned(), Expr::Int(262144));
        builtin_idents.insert("EX_VELADD".to_owned(), Expr::Int(524288));
        builtin_idents.insert("EX_BRIGHT".to_owned(), Expr::Int(1048576));
        builtin_idents.insert("EX_ACCELWEIRD".to_owned(), Expr::Int(2097152)); // Note: formerly EX_VELTIME
        builtin_idents.insert("EX_SIZE".to_owned(), Expr::Int(4194304));
        builtin_idents.insert("EX_SAVE".to_owned(), Expr::Int(8388608)); // Note: formerly EX_SAVEANGLE
        builtin_idents.insert("EX_ENMCREATE".to_owned(), Expr::Int(16777216)); // Note: formerly EX_SPECIAL
        builtin_idents.insert("EX_LAYER".to_owned(), Expr::Int(33554432));
        builtin_idents.insert("EX_DELAY".to_owned(), Expr::Int(67108864));
        builtin_idents.insert("EX_LASER".to_owned(), Expr::Int(134217728));
        builtin_idents.insert("EX_HITBOX".to_owned(), Expr::Int(536870912));
        builtin_idents.insert("EX_WAIT".to_owned(), Expr::Int(-2147483648));

        /* aim modes */
        /* AT = aim at player, ST = static aim */
        builtin_idents.insert("AIM_AT".to_owned(), Expr::Int(0));
        builtin_idents.insert("AIM_ST".to_owned(), Expr::Int(1));
        builtin_idents.insert("AIM_AT_RING".to_owned(), Expr::Int(2));
        builtin_idents.insert("AIM_ST_RING".to_owned(), Expr::Int(3));
        builtin_idents.insert("AIM_AWAY_RING".to_owned(), Expr::Int(4));
        builtin_idents.insert("AIM_ST_RING2".to_owned(), Expr::Int(5));
        builtin_idents.insert("AIM_RAND".to_owned(), Expr::Int(6));
        builtin_idents.insert("AIM_RAND_RING".to_owned(), Expr::Int(7));
        builtin_idents.insert("AIM_MEEK".to_owned(), Expr::Int(8));
        builtin_idents.insert("AIM_AT_PYRAMID".to_owned(), Expr::Int(9));
        builtin_idents.insert("AIM_ST_PYRAMID".to_owned(), Expr::Int(10));
        builtin_idents.insert("AIM_PEANUT".to_owned(), Expr::Int(11));
        builtin_idents.insert("AIM_PEANUT2".to_owned(), Expr::Int(12));
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
                if BUILTIN_IDENTS.contains_key(l) {
                    return Err(Error::Simple(format!(
                        "Can't use identifier {l} as a label"
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
