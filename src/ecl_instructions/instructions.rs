#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum ArgType {
    Int,
    IntRef,
    Float,
    FloatRef,
    Str,
    Varargs,
}

#[derive(Debug, Clone)]
pub struct InsDef {
    opcode: u16,
    alt_names: Vec<&'static str>,
    arg_format: Vec<ArgType>,
}

use crate::ast::{ExprType, Instr};

impl InsDef {
    pub fn varargs(&self) -> i32 {
        for (i, at) in self.arg_format.iter().enumerate() {
            if *at == ArgType::Varargs {
                return i as i32;
            }
        }
        -1
    }

    pub fn does_match(&self, ins: &Instr) -> bool {
        match ins {
            Instr::Call(name, exprs) => {
                // has same name:
                let has_same_name = if name.starts_with("ins_") {
                    let opcode = name.strip_prefix("ins_").unwrap().parse().unwrap();
                    self.opcode == opcode
                } else {
                    self.alt_names.contains(&&name[..])
                };
                if !has_same_name {
                    return false;
                }
                // check for varargs
                let va_pos = self.varargs();
                if va_pos >= 0 {
                    let self_args = &self.arg_format[..va_pos as usize];
                    let ins_args = &exprs[..va_pos as usize];
                    let ins_vargs = &exprs[va_pos as usize..];
                    if ins_args.len() != self_args.len() {
                        return false;
                    }
                    for varg in ins_vargs {
                        if varg.get_type() == ExprType::String {
                            return false;
                        }
                    }
                    for (sa, ia) in self_args.iter().zip(ins_args.iter()) {
                        match sa {
                            ArgType::Int => {
                                if ia.get_type() != ExprType::Int {
                                    return false;
                                }
                            }
                            ArgType::Float => {
                                if ia.get_type() != ExprType::Float {
                                    return false;
                                }
                            }
                            ArgType::Str => {
                                if ia.get_type() != ExprType::String {
                                    return false;
                                }
                            }
                            ArgType::IntRef => {
                                if ia.get_type() != ExprType::Int || !ia.is_var() {
                                    return false;
                                }
                            }
                            ArgType::FloatRef => {
                                if ia.get_type() != ExprType::Float || !ia.is_var() {
                                    return false;
                                }
                            }
                            ArgType::Varargs => {}
                        }
                    }
                    return true;
                }
                if exprs.len() != self.arg_format.len() {
                    return false;
                }
                for (sa, ia) in self.arg_format.iter().zip(exprs.iter()) {
                    match sa {
                        ArgType::Int => {
                            if ia.get_type() != ExprType::Int {
                                return false;
                            }
                        }
                        ArgType::Float => {
                            if ia.get_type() != ExprType::Float {
                                return false;
                            }
                        }
                        ArgType::Str => {
                            if ia.get_type() != ExprType::String {
                                return false;
                            }
                        }
                        ArgType::IntRef => {
                            if ia.get_type() != ExprType::Int || !ia.is_var() {
                                return false;
                            }
                        }
                        ArgType::FloatRef => {
                            if ia.get_type() != ExprType::Float || !ia.is_var() {
                                return false;
                            }
                        }
                        ArgType::Varargs => {}
                    }
                }
                return true;
            }
            _ => panic!("This is not an instruction call"),
        }
    }
}

pub fn matching_ins(ins: &Instr) -> u16 {
    for i in INSTRUCTION_SET.iter() {
        if i.does_match(ins) {
            return i.opcode;
        }
    }
    panic!("No such instruction: {:?}", ins);
}

pub fn matching_ins_sep(name: &str, expr: &Vec<crate::ast::Expr>) -> u16 {
    let i = Instr::Call(name.to_string(), expr.clone());
    matching_ins(&i)
}

lazy_static! {
    static ref INSTRUCTION_SET: Vec<InsDef> = {
        let v = vec![
        InsDef {
            opcode: 1,
            alt_names: vec![],
            arg_format: vec![],
        }, // delete is a keyword
        InsDef {
            opcode: 10,
            alt_names: vec![],
            arg_format: vec![],
        }, // return is a keyword
        InsDef {
            opcode: 11,
            alt_names: vec![],
            arg_format: vec![ArgType::Str, ArgType::Varargs],
        }, // use @ syntax
        InsDef {
            opcode: 12,
            alt_names: vec!["jmp"],
            arg_format: vec![ArgType::Int, ArgType::Float],
        },
        InsDef {
            opcode: 13,
            alt_names: vec!["jeq"],
            arg_format: vec![ArgType::Int, ArgType::Float],
        },
        InsDef {
            opcode: 14,
            alt_names: vec!["jne"],
            arg_format: vec![ArgType::Int, ArgType::Float],
        },
        InsDef {
            opcode: 15,
            alt_names: vec![],
            arg_format: vec![ArgType::Str, ArgType::Varargs],
        }, // @
        InsDef {
            opcode: 16,
            alt_names: vec![],
            arg_format: vec![ArgType::Str, ArgType::Int, ArgType::Varargs],
        }, // @
        InsDef {
            opcode: 17,
            alt_names: vec!["killAsync"],
            arg_format: vec![ArgType::Int],
        },
        InsDef {
            opcode: 18,
            alt_names: vec![],
            arg_format: vec![ArgType::Int],
        },
        InsDef {
            opcode: 19,
            alt_names: vec![],
            arg_format: vec![ArgType::Int],
        },
        InsDef {
            opcode: 20,
            alt_names: vec![],
            arg_format: vec![ArgType::Int, ArgType::Int],
        },
        InsDef {
            opcode: 21,
            alt_names: vec!["killAllAsync"],
            arg_format: vec![],
        },
        InsDef {
            opcode: 22,
            alt_names: vec![],
            arg_format: vec![ArgType::Int, ArgType::Str],
        },
        InsDef {
            opcode: 23,
            alt_names: vec!["wait"],
            arg_format: vec![ArgType::Int],
        },
        InsDef {
            opcode: 24,
            alt_names: vec!["wait"],
            arg_format: vec![ArgType::Float],
        },
        InsDef {
            opcode: 30,
            alt_names: vec!["printf"],
            arg_format: vec![ArgType::Str, ArgType::Varargs],
        },
        InsDef {
            opcode: 31,
            alt_names: vec![],
            arg_format: vec![],
        },
        InsDef {
            opcode: 40,
            alt_names: vec!["stackAlloc"],
            arg_format: vec![ArgType::Int],
        },
        InsDef {
            opcode: 41,
            alt_names: vec!["stackDealloc"],
            arg_format: vec![],
        },
        InsDef {
            opcode: 42,
            alt_names: vec!["push"],
            arg_format: vec![ArgType::Int],
        },
        InsDef {
            opcode: 43,
            alt_names: vec!["set"],
            arg_format: vec![ArgType::IntRef],
        },
        InsDef {
            opcode: 44,
            alt_names: vec!["push"],
            arg_format: vec![ArgType::Float],
        },
        InsDef {
            opcode: 45,
            alt_names: vec!["set"],
            arg_format: vec![ArgType::FloatRef],
        },
        InsDef {
            opcode: 50,
            alt_names: vec!["addi"],
            arg_format: vec![],
        },
        InsDef {
            opcode: 51,
            alt_names: vec!["addf"],
            arg_format: vec![],
        },
        InsDef {
            opcode: 52,
            alt_names: vec!["subi"],
            arg_format: vec![],
        },
        InsDef {
            opcode: 53,
            alt_names: vec!["subf"],
            arg_format: vec![],
        },
        InsDef {
            opcode: 54,
            alt_names: vec!["muli"],
            arg_format: vec![],
        },
        InsDef {
            opcode: 55,
            alt_names: vec!["mulf"],
            arg_format: vec![],
        },
        InsDef {
            opcode: 56,
            alt_names: vec!["divi"],
            arg_format: vec![],
        },
        InsDef {
            opcode: 57,
            alt_names: vec!["divf"],
            arg_format: vec![],
        },
        InsDef {
            opcode: 58,
            alt_names: vec!["modi"],
            arg_format: vec![],
        },
        InsDef {
            opcode: 59,
            alt_names: vec!["equi"],
            arg_format: vec![],
        },
        InsDef {
            opcode: 60,
            alt_names: vec!["equf"],
            arg_format: vec![],
        },
        InsDef {
            opcode: 61,
            alt_names: vec!["neqi"],
            arg_format: vec![],
        },
        InsDef {
            opcode: 62,
            alt_names: vec!["neqf"],
            arg_format: vec![],
        },
        InsDef {
            opcode: 63,
            alt_names: vec!["lesi"],
            arg_format: vec![],
        },
        InsDef {
            opcode: 64,
            alt_names: vec!["lesf"],
            arg_format: vec![],
        },
        InsDef {
            opcode: 65,
            alt_names: vec!["leqi"],
            arg_format: vec![],
        },
        InsDef {
            opcode: 66,
            alt_names: vec!["leqf"],
            arg_format: vec![],
        },
        InsDef {
            opcode: 67,
            alt_names: vec!["grei"],
            arg_format: vec![],
        },
        InsDef {
            opcode: 68,
            alt_names: vec!["gref"],
            arg_format: vec![],
        },
        InsDef {
            opcode: 69,
            alt_names: vec!["geqi"],
            arg_format: vec![],
        },
        InsDef {
            opcode: 70,
            alt_names: vec!["geqf"],
            arg_format: vec![],
        },
        InsDef {
            opcode: 71,
            alt_names: vec!["noti"],
            arg_format: vec![],
        },
        InsDef {
            opcode: 72,
            alt_names: vec!["notf"],
            arg_format: vec![],
        },
        InsDef {
            opcode: 73,
            alt_names: vec!["or"],
            arg_format: vec![],
        },
        InsDef {
            opcode: 74,
            alt_names: vec!["and"],
            arg_format: vec![],
        },
        InsDef {
            opcode: 75,
            alt_names: vec!["xor"],
            arg_format: vec![],
        },
        InsDef {
            opcode: 76,
            alt_names: vec!["bor"],
            arg_format: vec![],
        },
        InsDef {
            opcode: 77,
            alt_names: vec!["band"],
            arg_format: vec![],
        },
        InsDef {
            opcode: 78,
            alt_names: vec!["deci"],
            arg_format: vec![ArgType::IntRef],
        },
        InsDef {
            opcode: 79,
            alt_names: vec!["ssin"],
            arg_format: vec![],
        },
        InsDef {
            opcode: 80,
            alt_names: vec!["scos"],
            arg_format: vec![],
        },
        InsDef {
            opcode: 83,
            alt_names: vec!["negi"],
            arg_format: vec![],
        },
        InsDef {
            opcode: 84,
            alt_names: vec!["negf"],
            arg_format: vec![],
        },
        InsDef {
            opcode: 88,
            alt_names: vec!["sqrt"],
            arg_format: vec![],
        },
        InsDef {
            opcode: 82,
            alt_names: vec!["circlePos"],
            arg_format: vec![ArgType::FloatRef, ArgType::FloatRef, ArgType::Float, ArgType::Float],
        },
        InsDef {
            opcode: 82,
            alt_names: vec!["validRad"],
            arg_format: vec![ArgType::FloatRef],
        },
        InsDef {
            opcode: 85,
            alt_names: vec!["sqSum"],
            arg_format: vec![ArgType::FloatRef, ArgType::Float, ArgType::Float],
        },
        InsDef {
            opcode: 86,
            alt_names: vec!["sqSumRt"],
            arg_format: vec![ArgType::FloatRef, ArgType::Float, ArgType::Float],
        },
        InsDef {
            opcode: 87,
            alt_names: vec!["getAng"],
            arg_format: vec![ArgType::FloatRef, ArgType::Float, ArgType::Float, ArgType::Float, ArgType::Float],
        },
        InsDef {
            opcode: 89,
            alt_names: vec!["linFunc"],
            arg_format: vec![ArgType::FloatRef, ArgType::Float, ArgType::Float],
        },
        InsDef {
            opcode: 90,
            alt_names: vec!["ptRot"],
            arg_format: vec![ArgType::FloatRef, ArgType::FloatRef, ArgType::Float, ArgType::Float, ArgType::Float],
        },
        InsDef {
            opcode: 91,
            alt_names: vec!["floatTime"],
            arg_format: vec![ArgType::Int, ArgType::FloatRef, ArgType::Int, ArgType::Int, ArgType::Float, ArgType::Float],
        },
        InsDef {
            opcode: 92,
            alt_names: vec!["floatTimeEx"],
            arg_format: vec![ArgType::Int, ArgType::FloatRef, ArgType::Int, ArgType::Int, ArgType::Float, ArgType::Float, ArgType::Float, ArgType::Float],
        },
        InsDef {
            opcode: 93,
            alt_names: vec!["randRadius"],
            arg_format: vec![ArgType::FloatRef, ArgType::FloatRef, ArgType::Float, ArgType::Float],
        },

        InsDef {
            opcode: 300,
            alt_names: vec!["enmCreate"],
            arg_format: vec![ArgType::Str, ArgType::Float, ArgType::Float, ArgType::Int, ArgType::Int, ArgType::Int],
        },
        InsDef {
            opcode: 301,
            alt_names: vec!["enmCreateA"],
            arg_format: vec![ArgType::Str, ArgType::Float, ArgType::Float, ArgType::Int, ArgType::Int, ArgType::Int],
        },
        InsDef {
            opcode: 302,
            alt_names: vec!["anmSelect"],
            arg_format: vec![ArgType::Int],
        },
        InsDef {
            opcode: 303,
            alt_names: vec!["anmSetSpr"],
            arg_format: vec![ArgType::Int, ArgType::Int],
        },
        InsDef {
            opcode: 304,
            alt_names: vec!["enmCreateM"],
            arg_format: vec![ArgType::Str, ArgType::Float, ArgType::Float, ArgType::Int, ArgType::Int, ArgType::Int],
        },
        InsDef {
            opcode: 305,
            alt_names: vec!["enmCreateAM"],
            arg_format: vec![ArgType::Str, ArgType::Float, ArgType::Float, ArgType::Int, ArgType::Int, ArgType::Int],
        },
        InsDef {
            opcode: 306,
            alt_names: vec!["anmSetMain"],
            arg_format: vec![ArgType::Int, ArgType::Int],
        },
        InsDef {
            opcode: 307,
            alt_names: vec!["anmPlay"],
            arg_format: vec![ArgType::Int, ArgType::Int],
        },
        InsDef {
            opcode: 308,
            alt_names: vec!["anmPlayAbs"],
            arg_format: vec![ArgType::Int, ArgType::Int],
        },
        InsDef {
            opcode: 309,
            alt_names: vec!["enmCreateF"],
            arg_format: vec![ArgType::Str, ArgType::Float, ArgType::Float, ArgType::Int, ArgType::Int, ArgType::Int],
        },
        InsDef {
            opcode: 310,
            alt_names: vec!["enmCreateAF"],
            arg_format: vec![ArgType::Str, ArgType::Float, ArgType::Float, ArgType::Int, ArgType::Int, ArgType::Int],
        },
        InsDef {
            opcode: 311,
            alt_names: vec!["enmCreateMF"],
            arg_format: vec![ArgType::Str, ArgType::Float, ArgType::Float, ArgType::Int, ArgType::Int, ArgType::Int],
        },
        InsDef {
            opcode: 312,
            alt_names: vec!["enmCreateAMF"],
            arg_format: vec![ArgType::Str, ArgType::Float, ArgType::Float, ArgType::Int, ArgType::Int, ArgType::Int],
        },
        InsDef {
            opcode: 313,
            alt_names: vec!["anmSelPlay"],
            arg_format: vec![ArgType::Int],
        },
        InsDef {
            opcode: 314,
            alt_names: vec!["anmPlayHigh"],
            arg_format: vec![ArgType::Int, ArgType::Int],
        },
        InsDef {
            opcode: 315,
            alt_names: vec!["anmPlayRotate"],
            arg_format: vec![ArgType::Int, ArgType::Int, ArgType::Float],
        },
        InsDef {
            opcode: 316,
            alt_names: vec!["anm316"],
            arg_format: vec![ArgType::Int, ArgType::Int],
        },
        InsDef {
            opcode: 317,
            alt_names: vec!["anmSwitch"],
            arg_format: vec![ArgType::Int, ArgType::Int],
        },
        InsDef {
            opcode: 318,
            alt_names: vec!["anmReset"],
            arg_format: vec![],
        },
        InsDef {
            opcode: 319,
            alt_names: vec!["anmRotate"],
            arg_format: vec![ArgType::Int, ArgType::Float],
        },
        InsDef {
            opcode: 320,
            alt_names: vec!["anmMove"],
            arg_format: vec![ArgType::Int, ArgType::Float, ArgType::Float],
        },
        InsDef {
            opcode: 321,
            alt_names: vec!["enmMapleEnemy"],
            arg_format: vec![ArgType::Str, ArgType::Float, ArgType::Float, ArgType::Int, ArgType::Int, ArgType::Int],
        },
        InsDef {
            opcode: 322,
            alt_names: vec!["enm322"],
            arg_format: vec![ArgType::Int, ArgType::Int],
        },
        InsDef {
            opcode: 323,
            alt_names: vec!["deathAnm"],
            arg_format: vec![ArgType::Int, ArgType::Int],
        },
        InsDef {
            opcode: 324,
            alt_names: vec!["enmPos2"],
            arg_format: vec![ArgType::FloatRef, ArgType::FloatRef, ArgType::Int],
        },
        InsDef {
            opcode: 325,
            alt_names: vec!["anmCol"],
            arg_format: vec![ArgType::Int, ArgType::Int, ArgType::Int, ArgType::Int],
        },
        InsDef {
            opcode: 326,
            alt_names: vec!["anmColT"],
            arg_format: vec![ArgType::Int, ArgType::Int, ArgType::Int, ArgType::Int, ArgType::Int, ArgType::Int],
        },
        InsDef {
            opcode: 327,
            alt_names: vec!["anmAlpha"],
            arg_format: vec![ArgType::Int, ArgType::Int],
        },
        InsDef {
            opcode: 328,
            alt_names: vec!["anmAlphaT"],
            arg_format: vec![ArgType::Int, ArgType::Int, ArgType::Int, ArgType::Int],
        },
        InsDef {
            opcode: 329,
            alt_names: vec!["anmScale"],
            arg_format: vec![ArgType::Int, ArgType::Float, ArgType::Float],
        },
        InsDef {
            opcode: 330,
            alt_names: vec!["anmScaleT"],
            arg_format: vec![ArgType::Int, ArgType::Int, ArgType::Int, ArgType::Float, ArgType::Float],
        },
        InsDef {
            opcode: 331,
            alt_names: vec!["anmAlpha2"],
            arg_format: vec![ArgType::Int, ArgType::Int],
        },
        InsDef {
            opcode: 332,
            alt_names: vec!["anmAlpha2T"],
            arg_format: vec![ArgType::Int, ArgType::Int, ArgType::Int, ArgType::Int],
        },
        InsDef {
            opcode: 333,
            alt_names: vec!["anmPosT"],
            arg_format: vec![ArgType::Int, ArgType::Int, ArgType::Int, ArgType::Float, ArgType::Float],
        },
        InsDef {
            opcode: 334,
            alt_names: vec!["anm334"],
            arg_format: vec![ArgType::Int],
        },
        InsDef {
            opcode: 335,
            alt_names: vec!["anmScale2"],
            arg_format: vec![ArgType::Int, ArgType::Float, ArgType::Float],
        },
        InsDef {
            opcode: 336,
            alt_names: vec!["anmLayer"],
            arg_format: vec![ArgType::Int, ArgType::Int],
        },
        InsDef {
            opcode: 337,
            alt_names: vec!["anmBM_16_anmPlayPos"],
            arg_format: vec![ArgType::Int, ArgType::Int],
        },
        InsDef {
            opcode: 338,
            alt_names: vec!["anmPlayPos"],
            arg_format: vec![ArgType::Int, ArgType::Int, ArgType::Float, ArgType::Float, ArgType::Float],
        },
        InsDef {
            opcode: 339,
            alt_names: vec!["anm339"],
            arg_format: vec![ArgType::Int, ArgType::Int, ArgType::Int],
        },
        InsDef {
            opcode: 340,
            alt_names: vec!["enmDelete"],
            arg_format: vec![ArgType::Int],
        },

        InsDef {
            opcode: 400,
            alt_names: vec!["movePos"],
            arg_format: vec![ArgType::Float, ArgType::Float]
        },
        InsDef {
            opcode: 401,
            alt_names: vec!["movePosTime"],
            arg_format: vec![ArgType::Int, ArgType::Int, ArgType::Float, ArgType::Float]
        },
        InsDef {
            opcode: 402,
            alt_names: vec!["movePosRel"],
            arg_format: vec![ArgType::Float, ArgType::Float]
        },
        InsDef {
            opcode: 403,
            alt_names: vec!["movePosRelTime"],
            arg_format: vec![ArgType::Int, ArgType::Int, ArgType::Float, ArgType::Float]
        },
        InsDef {
            opcode: 404,
            alt_names: vec!["moveVel"],
            arg_format: vec![ArgType::Float, ArgType::Float]
        },
        InsDef {
            opcode: 405,
            alt_names: vec!["moveVelTime"],
            arg_format: vec![ArgType::Int, ArgType::Int, ArgType::Float, ArgType::Float]
        },
        InsDef {
            opcode: 406,
            alt_names: vec!["moveVelRel"],
            arg_format: vec![ArgType::Float, ArgType::Float]
        },
        InsDef {
            opcode: 407,
            alt_names: vec!["moveVelRelTime"],
            arg_format: vec![ArgType::Int, ArgType::Int, ArgType::Float, ArgType::Float]
        },
        InsDef {
            opcode: 408,
            alt_names: vec!["moveCirc"],
            arg_format: vec![ArgType::Float, ArgType::Float, ArgType::Float, ArgType::Float]
        },
        InsDef {
            opcode: 409,
            alt_names: vec!["moveCircTime"],
            arg_format: vec![ArgType::Int, ArgType::Int, ArgType::Float, ArgType::Float, ArgType::Float]
        },
        InsDef {
            opcode: 410,
            alt_names: vec!["moveCircRel"],
            arg_format: vec![ArgType::Float, ArgType::Float, ArgType::Float, ArgType::Float]
        },
        InsDef {
            opcode: 411,
            alt_names: vec!["moveCircRelTime"],
            arg_format: vec![ArgType::Int, ArgType::Int, ArgType::Float, ArgType::Float, ArgType::Float]
        },
        InsDef {
            opcode: 412,
            alt_names: vec!["moveRand"],
            arg_format: vec![ArgType::Int, ArgType::Int, ArgType::Float]
        },
        InsDef {
            opcode: 413,
            alt_names: vec!["moveRandRel"],
            arg_format: vec![ArgType::Int, ArgType::Int, ArgType::Float]
        },
        InsDef {
            opcode: 414,
            alt_names: vec!["moveBoss"],
            arg_format: vec![]
        },
        InsDef {
            opcode: 415,
            alt_names: vec!["moveBossRel"],
            arg_format: vec![]
        },
        InsDef {
            opcode: 416,
            alt_names: vec!["movePos3d"],
            arg_format: vec![ArgType::Float, ArgType::Float, ArgType::Float]
        },
        InsDef {
            opcode: 417,
            alt_names: vec!["movePos3dRel"],
            arg_format: vec![ArgType::Float, ArgType::Float, ArgType::Float]
        },
        InsDef {
            opcode: 418,
            alt_names: vec!["moveAdd"],
            arg_format: vec![ArgType::Float, ArgType::Float]
        },
        InsDef {
            opcode: 419,
            alt_names: vec!["moveAddRel"],
            arg_format: vec![ArgType::Float, ArgType::Float]
        },
        InsDef {
            opcode: 420,
            alt_names: vec!["moveEll"],
            arg_format: vec![ArgType::Float, ArgType::Float, ArgType::Float, ArgType::Float, ArgType::Float, ArgType::Float]
        },
        InsDef {
            opcode: 421,
            alt_names: vec!["moveEllTime"],
            arg_format: vec![ArgType::Int, ArgType::Int, ArgType::Float, ArgType::Float, ArgType::Float, ArgType::Float, ArgType::Float]
        },
        InsDef {
            opcode: 422,
            alt_names: vec!["moveEllRel"],
            arg_format: vec![ArgType::Float, ArgType::Float, ArgType::Float, ArgType::Float, ArgType::Float, ArgType::Float]
        },
        InsDef {
            opcode: 423,
            alt_names: vec!["moveEllRelTime"],
            arg_format: vec![ArgType::Int, ArgType::Int, ArgType::Float, ArgType::Float, ArgType::Float, ArgType::Float, ArgType::Float]
        },
        InsDef {
            opcode: 424,
            alt_names: vec!["moveMirror"],
            arg_format: vec![ArgType::Int]
        },
        InsDef {
            opcode: 425,
            alt_names: vec!["moveBezier"],
            arg_format: vec![ArgType::Int, ArgType::Float, ArgType::Float, ArgType::Float, ArgType::Float, ArgType::Float, ArgType::Float]
        },
        InsDef {
            opcode: 426,
            alt_names: vec!["moveBezierRel"],
            arg_format: vec![ArgType::Int, ArgType::Float, ArgType::Float, ArgType::Float, ArgType::Float, ArgType::Float, ArgType::Float]
        },
        InsDef {
            opcode: 427,
            alt_names: vec!["moveReset"],
            arg_format: vec![]
        },
        InsDef {
            opcode: 428,
            alt_names: vec!["moveVelNM"],
            arg_format: vec![ArgType::Float, ArgType::Float]
        },
        InsDef {
            opcode: 429,
            alt_names: vec!["moveVelTimeNM"],
            arg_format: vec![ArgType::Int, ArgType::Int, ArgType::Float, ArgType::Float]
        },
        InsDef {
            opcode: 430,
            alt_names: vec!["moveVelRelNM"],
            arg_format: vec![ArgType::Float, ArgType::Float]
        },
        InsDef {
            opcode: 431,
            alt_names: vec!["moveVelRelTimeNM"],
            arg_format: vec![ArgType::Int, ArgType::Int, ArgType::Float, ArgType::Float]
        },
        InsDef {
            opcode: 432,
            alt_names: vec!["moveEnm"],
            arg_format: vec![ArgType::Int]
        },
        InsDef {
            opcode: 433,
            alt_names: vec!["moveEnmRel"],
            arg_format: vec![ArgType::Int]
        },
        InsDef {
            opcode: 434,
            alt_names: vec!["moveCurve"],
            arg_format: vec![ArgType::Int, ArgType::Int, ArgType::Int, ArgType::Float, ArgType::Float]
        },
        InsDef {
            opcode: 435,
            alt_names: vec!["moveCurveRel"],
            arg_format: vec![ArgType::Int, ArgType::Int, ArgType::Int, ArgType::Float, ArgType::Float]
        },
        InsDef {
            opcode: 436,
            alt_names: vec!["moveAddTime"],
            arg_format: vec![ArgType::Int, ArgType::Int, ArgType::Float, ArgType::Float]
        },
        InsDef {
            opcode: 437,
            alt_names: vec!["moveAddRelTime"],
            arg_format: vec![ArgType::Int, ArgType::Int, ArgType::Float, ArgType::Float]
        },
        InsDef {
            opcode: 438,
            alt_names: vec!["moveCurveAdd"],
            arg_format: vec![ArgType::Int, ArgType::Int, ArgType::Int, ArgType::Float, ArgType::Float]
        },
        InsDef {
            opcode: 439,
            alt_names: vec!["moveCurveAddRel"],
            arg_format: vec![ArgType::Int, ArgType::Int, ArgType::Int, ArgType::Float, ArgType::Float]
        },
        InsDef {
            opcode: 440,
            alt_names: vec!["moveAngle"],
            arg_format: vec![ArgType::Float]
        },
        InsDef {
            opcode: 441,
            alt_names: vec!["moveAngleTime"],
            arg_format: vec![ArgType::Int, ArgType::Int, ArgType::Float]
        },
        InsDef {
            opcode: 442,
            alt_names: vec!["moveAngleRel"],
            arg_format: vec![ArgType::Float]
        },
        InsDef {
            opcode: 443,
            alt_names: vec!["moveAngleRelTime"],
            arg_format: vec![ArgType::Int, ArgType::Int, ArgType::Float]
        },
        InsDef {
            opcode: 444,
            alt_names: vec!["moveSpeed"],
            arg_format: vec![ArgType::Float]
        },
        InsDef {
            opcode: 445,
            alt_names: vec!["moveSpeedTime"],
            arg_format: vec![ArgType::Int, ArgType::Int, ArgType::Float]
        },
        InsDef {
            opcode: 446,
            alt_names: vec!["moveSpeedRel"],
            arg_format: vec![ArgType::Float],
        },
        InsDef {
            opcode: 447,
            alt_names: vec!["moveSpeedRelTime"],
            arg_format: vec![ArgType::Int, ArgType::Int, ArgType::Float]
        },
    ];
    v
    };
}
