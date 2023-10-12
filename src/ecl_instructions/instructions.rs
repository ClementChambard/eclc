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

use crate::{
    ast::{Expr, ExprType},
    error::Error,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MatchType {
    PerfectMatch,
    WithVarargs(usize),
    StringInVarargs,
    NameAndArgCountMatch,
    NameMatch,
    NoMatch,
}

impl InsDef {
    pub fn signature(&self) -> String {
        let mut s = format!("ins_{}", self.opcode);
        for alt in &self.alt_names {
            s.push('/');
            s.push_str(alt);
        }
        s.push('(');
        for (i, a) in self.arg_format.iter().enumerate() {
            if i != 0 {
                s.push_str(", ");
            }
            match a {
                ArgType::Int => s.push_str("int"),
                ArgType::IntRef => s.push_str("int&"),
                ArgType::Float => s.push_str("float"),
                ArgType::FloatRef => s.push_str("float&"),
                ArgType::Str => s.push_str("str"),
                ArgType::Varargs => s.push_str("..."),
            }
        }
        s.push(')');
        s
    }

    pub fn varargs(&self) -> i32 {
        for (i, at) in self.arg_format.iter().enumerate() {
            if *at == ArgType::Varargs {
                return i as i32;
            }
        }
        -1
    }

    pub fn does_match(&self, name: &str, exprs: &Vec<Expr>) -> Result<MatchType, Error> {
        // has same name:
        let has_same_name = if name.starts_with("ins_") {
            let opcode = name.strip_prefix("ins_").unwrap().parse().unwrap();
            self.opcode == opcode
        } else {
            self.alt_names.contains(&name)
        };
        if !has_same_name {
            return Ok(MatchType::NoMatch);
        }
        // check for varargs
        let va_pos = self.varargs();
        if va_pos >= 0 {
            let va_pos = va_pos as usize;
            if va_pos > exprs.len() {
                return Ok(MatchType::NameMatch);
            }
            let self_args = &self.arg_format[..va_pos];
            let ins_args = &exprs[..va_pos];
            let ins_vargs = &exprs[va_pos..];
            if ins_args.len() != self_args.len() {
                return Ok(MatchType::NameMatch);
            }
            for varg in ins_vargs {
                if varg.get_type()? == ExprType::String {
                    return Ok(MatchType::StringInVarargs);
                }
            }
            for (sa, ia) in self_args.iter().zip(ins_args.iter()) {
                match sa {
                    ArgType::Int => {
                        if ia.get_type()? != ExprType::Int {
                            return Ok(MatchType::NameAndArgCountMatch);
                        }
                    }
                    ArgType::Float => {
                        if ia.get_type()? != ExprType::Float {
                            return Ok(MatchType::NameAndArgCountMatch);
                        }
                    }
                    ArgType::Str => {
                        if ia.get_type()? != ExprType::String {
                            return Ok(MatchType::NameAndArgCountMatch);
                        }
                    }
                    ArgType::IntRef => {
                        if ia.get_type()? != ExprType::Int || !ia.is_var() {
                            return Ok(MatchType::NameAndArgCountMatch);
                        }
                    }
                    ArgType::FloatRef => {
                        if ia.get_type()? != ExprType::Float {
                            return Ok(MatchType::NameAndArgCountMatch);
                        }
                        if !ia.is_var() {}
                    }
                    ArgType::Varargs => {}
                }
            }
            return Ok(MatchType::WithVarargs(va_pos));
        }
        if exprs.len() != self.arg_format.len() {
            return Ok(MatchType::NameMatch);
        }
        for (sa, ia) in self.arg_format.iter().zip(exprs.iter()) {
            match sa {
                ArgType::Int => {
                    if ia.get_type()? != ExprType::Int {
                        return Ok(MatchType::NameAndArgCountMatch);
                    }
                }
                ArgType::Float => {
                    if ia.get_type()? != ExprType::Float {
                        return Ok(MatchType::NameAndArgCountMatch);
                    }
                }
                ArgType::Str => {
                    if ia.get_type()? != ExprType::String {
                        return Ok(MatchType::NameAndArgCountMatch);
                    }
                }
                ArgType::IntRef => {
                    if ia.get_type()? != ExprType::Int || !ia.is_var() {
                        return Ok(MatchType::NameAndArgCountMatch);
                    }
                }
                ArgType::FloatRef => {
                    if ia.get_type()? != ExprType::Float || !ia.is_var() {
                        return Ok(MatchType::NameAndArgCountMatch);
                    }
                }
                ArgType::Varargs => {}
            }
        }
        Ok(MatchType::PerfectMatch)
    }
}

#[derive(Debug, Clone)]
pub enum MatchInsResult {
    Match(u16),
    MatchVA(u16, usize),
    NoMatch(Vec<NearMatch>),
}

#[derive(Debug, Clone)]
pub struct NearMatch {
    pub id: &'static InsDef,
    pub mt: MatchType,
}

pub fn matching_ins_sep(name: &str, expr: &Vec<crate::ast::Expr>) -> Result<MatchInsResult, Error> {
    let mut near_matches = Vec::new();
    for i in INSTRUCTION_SET.iter() {
        let matching = i.does_match(name, expr)?;
        match matching {
            MatchType::PerfectMatch => return Ok(MatchInsResult::Match(i.opcode)),
            MatchType::WithVarargs(va) => return Ok(MatchInsResult::MatchVA(i.opcode, va)),
            MatchType::NoMatch => {}
            _ => near_matches.push(NearMatch {
                id: i,
                mt: matching,
            }),
        }
    }
    Ok(MatchInsResult::NoMatch(near_matches))
}

lazy_static! {
    static ref INSTRUCTION_SET: Vec<InsDef> = {
        use ArgType as A;
        let v = vec![
        InsDef { opcode: 1, alt_names: vec![], arg_format: vec![], }, // delete is a keyword
        InsDef { opcode: 10, alt_names: vec![], arg_format: vec![], }, // return is a keyword
        InsDef { opcode: 11, alt_names: vec![], arg_format: vec![A::Str, A::Varargs], }, // use @ syntax
        InsDef { opcode: 12, alt_names: vec!["jmp"], arg_format: vec![A::Int, A::Float], },
        InsDef { opcode: 13, alt_names: vec!["jeq"], arg_format: vec![A::Int, A::Float], },
        InsDef { opcode: 14, alt_names: vec!["jne"], arg_format: vec![A::Int, A::Float], },
        InsDef { opcode: 15, alt_names: vec![], arg_format: vec![A::Str, A::Varargs], }, // @
        InsDef { opcode: 16, alt_names: vec![], arg_format: vec![A::Str, A::Int, A::Varargs], }, // @
        InsDef { opcode: 17, alt_names: vec!["killAsync"], arg_format: vec![A::Int], },
        InsDef { opcode: 18, alt_names: vec![], arg_format: vec![A::Int], },
        InsDef { opcode: 19, alt_names: vec![], arg_format: vec![A::Int], },
        InsDef { opcode: 20, alt_names: vec![], arg_format: vec![A::Int, A::Int], },
        InsDef { opcode: 21, alt_names: vec!["killAllAsync"], arg_format: vec![], },
        InsDef { opcode: 22, alt_names: vec![], arg_format: vec![A::Int, A::Str], },
        InsDef { opcode: 23, alt_names: vec!["wait"], arg_format: vec![A::Int], },
        InsDef { opcode: 24, alt_names: vec!["wait"], arg_format: vec![A::Float], },
        InsDef { opcode: 30, alt_names: vec!["printf"], arg_format: vec![A::Str, A::Varargs], },
        InsDef { opcode: 31, alt_names: vec![], arg_format: vec![], },
        InsDef { opcode: 40, alt_names: vec!["stackAlloc"], arg_format: vec![A::Int], },
        InsDef { opcode: 41, alt_names: vec!["stackDealloc"], arg_format: vec![], },
        InsDef { opcode: 42, alt_names: vec!["push"], arg_format: vec![A::Int], },
        InsDef { opcode: 43, alt_names: vec!["set"], arg_format: vec![A::IntRef], },
        InsDef { opcode: 44, alt_names: vec!["push"], arg_format: vec![A::Float], },
        InsDef { opcode: 45, alt_names: vec!["set"], arg_format: vec![A::FloatRef], },
        InsDef { opcode: 50, alt_names: vec!["addi"], arg_format: vec![], },
        InsDef { opcode: 51, alt_names: vec!["addf"], arg_format: vec![], },
        InsDef { opcode: 52, alt_names: vec!["subi"], arg_format: vec![], },
        InsDef { opcode: 53, alt_names: vec!["subf"], arg_format: vec![], },
        InsDef { opcode: 54, alt_names: vec!["muli"], arg_format: vec![], },
        InsDef { opcode: 55, alt_names: vec!["mulf"], arg_format: vec![], },
        InsDef { opcode: 56, alt_names: vec!["divi"], arg_format: vec![], },
        InsDef { opcode: 57, alt_names: vec!["divf"], arg_format: vec![], },
        InsDef { opcode: 58, alt_names: vec!["modi"], arg_format: vec![], },
        InsDef { opcode: 59, alt_names: vec!["equi"], arg_format: vec![], },
        InsDef { opcode: 60, alt_names: vec!["equf"], arg_format: vec![], },
        InsDef { opcode: 61, alt_names: vec!["neqi"], arg_format: vec![], },
        InsDef { opcode: 62, alt_names: vec!["neqf"], arg_format: vec![], },
        InsDef { opcode: 63, alt_names: vec!["lesi"], arg_format: vec![], },
        InsDef { opcode: 64, alt_names: vec!["lesf"], arg_format: vec![], },
        InsDef { opcode: 65, alt_names: vec!["leqi"], arg_format: vec![], },
        InsDef { opcode: 66, alt_names: vec!["leqf"], arg_format: vec![], },
        InsDef { opcode: 67, alt_names: vec!["grei"], arg_format: vec![], },
        InsDef { opcode: 68, alt_names: vec!["gref"], arg_format: vec![], },
        InsDef { opcode: 69, alt_names: vec!["geqi"], arg_format: vec![], },
        InsDef { opcode: 70, alt_names: vec!["geqf"], arg_format: vec![], },
        InsDef { opcode: 71, alt_names: vec!["noti"], arg_format: vec![], },
        InsDef { opcode: 72, alt_names: vec!["notf"], arg_format: vec![], },
        InsDef { opcode: 73, alt_names: vec!["or"], arg_format: vec![], },
        InsDef { opcode: 74, alt_names: vec!["and"], arg_format: vec![], },
        InsDef { opcode: 75, alt_names: vec!["xor"], arg_format: vec![], },
        InsDef { opcode: 76, alt_names: vec!["bor"], arg_format: vec![], },
        InsDef { opcode: 77, alt_names: vec!["band"], arg_format: vec![], },
        InsDef { opcode: 78, alt_names: vec!["deci"], arg_format: vec![A::IntRef], },
        InsDef { opcode: 79, alt_names: vec!["ssin"], arg_format: vec![], },
        InsDef { opcode: 80, alt_names: vec!["scos"], arg_format: vec![], },
        InsDef { opcode: 83, alt_names: vec!["negi"], arg_format: vec![], },
        InsDef { opcode: 84, alt_names: vec!["negf"], arg_format: vec![], },
        InsDef { opcode: 88, alt_names: vec!["sqrt"], arg_format: vec![], },
        InsDef { opcode: 82, alt_names: vec!["circlePos"], arg_format: vec![A::FloatRef, A::FloatRef, A::Float, A::Float], },
        InsDef { opcode: 82, alt_names: vec!["validRad"], arg_format: vec![A::FloatRef], },
        InsDef { opcode: 85, alt_names: vec!["sqSum"], arg_format: vec![A::FloatRef, A::Float, A::Float], },
        InsDef { opcode: 86, alt_names: vec!["sqSumRt"], arg_format: vec![A::FloatRef, A::Float, A::Float], },
        InsDef { opcode: 87, alt_names: vec!["getAng"], arg_format: vec![A::FloatRef, A::Float, A::Float, A::Float, A::Float], },
        InsDef { opcode: 89, alt_names: vec!["linFunc"], arg_format: vec![A::FloatRef, A::Float, A::Float], },
        InsDef { opcode: 90, alt_names: vec!["ptRot"], arg_format: vec![A::FloatRef, A::FloatRef, A::Float, A::Float, A::Float], },
        InsDef { opcode: 91, alt_names: vec!["floatTime"], arg_format: vec![A::Int, A::FloatRef, A::Int, A::Int, A::Float, A::Float], },
        InsDef { opcode: 92, alt_names: vec!["floatTimeEx"], arg_format: vec![A::Int, A::FloatRef, A::Int, A::Int, A::Float, A::Float, A::Float, A::Float], },
        InsDef { opcode: 93, alt_names: vec!["randRadius"], arg_format: vec![A::FloatRef, A::FloatRef, A::Float, A::Float], },

        InsDef { opcode: 300, alt_names: vec!["enmCreate"], arg_format: vec![A::Str, A::Float, A::Float, A::Int, A::Int, A::Int], },
        InsDef { opcode: 301, alt_names: vec!["enmCreateA"], arg_format: vec![A::Str, A::Float, A::Float, A::Int, A::Int, A::Int], },
        InsDef { opcode: 302, alt_names: vec!["anmSelect"], arg_format: vec![A::Int], },
        InsDef { opcode: 303, alt_names: vec!["anmSetSpr"], arg_format: vec![A::Int, A::Int], },
        InsDef { opcode: 304, alt_names: vec!["enmCreateM"], arg_format: vec![A::Str, A::Float, A::Float, A::Int, A::Int, A::Int], },
        InsDef { opcode: 305, alt_names: vec!["enmCreateAM"], arg_format: vec![A::Str, A::Float, A::Float, A::Int, A::Int, A::Int], },
        InsDef { opcode: 306, alt_names: vec!["anmSetMain"], arg_format: vec![A::Int, A::Int], },
        InsDef { opcode: 307, alt_names: vec!["anmPlay"], arg_format: vec![A::Int, A::Int], },
        InsDef { opcode: 308, alt_names: vec!["anmPlayAbs"], arg_format: vec![A::Int, A::Int], },
        InsDef { opcode: 309, alt_names: vec!["enmCreateF"], arg_format: vec![A::Str, A::Float, A::Float, A::Int, A::Int, A::Int], },
        InsDef { opcode: 310, alt_names: vec!["enmCreateAF"], arg_format: vec![A::Str, A::Float, A::Float, A::Int, A::Int, A::Int], },
        InsDef { opcode: 311, alt_names: vec!["enmCreateMF"], arg_format: vec![A::Str, A::Float, A::Float, A::Int, A::Int, A::Int], },
        InsDef { opcode: 312, alt_names: vec!["enmCreateAMF"], arg_format: vec![A::Str, A::Float, A::Float, A::Int, A::Int, A::Int], },
        InsDef { opcode: 313, alt_names: vec!["anmSelPlay"], arg_format: vec![A::Int], },
        InsDef { opcode: 314, alt_names: vec!["anmPlayHigh"], arg_format: vec![A::Int, A::Int], },
        InsDef { opcode: 315, alt_names: vec!["anmPlayRotate"], arg_format: vec![A::Int, A::Int, A::Float], },
        InsDef { opcode: 316, alt_names: vec!["anm316"], arg_format: vec![A::Int, A::Int], },
        InsDef { opcode: 317, alt_names: vec!["anmSwitch"], arg_format: vec![A::Int, A::Int], },
        InsDef { opcode: 318, alt_names: vec!["anmReset"], arg_format: vec![], },
        InsDef { opcode: 319, alt_names: vec!["anmRotate"], arg_format: vec![A::Int, A::Float], },
        InsDef { opcode: 320, alt_names: vec!["anmMove"], arg_format: vec![A::Int, A::Float, A::Float], },
        InsDef { opcode: 321, alt_names: vec!["enmMapleEnemy"], arg_format: vec![A::Str, A::Float, A::Float, A::Int, A::Int, A::Int], },
        InsDef { opcode: 322, alt_names: vec!["enm322"], arg_format: vec![A::Int, A::Int], },
        InsDef { opcode: 323, alt_names: vec!["deathAnm"], arg_format: vec![A::Int, A::Int], },
        InsDef { opcode: 324, alt_names: vec!["enmPos2"], arg_format: vec![A::FloatRef, A::FloatRef, A::Int], },
        InsDef { opcode: 325, alt_names: vec!["anmCol"], arg_format: vec![A::Int, A::Int, A::Int, A::Int], },
        InsDef { opcode: 326, alt_names: vec!["anmColT"], arg_format: vec![A::Int, A::Int, A::Int, A::Int, A::Int, A::Int], },
        InsDef { opcode: 327, alt_names: vec!["anmAlpha"], arg_format: vec![A::Int, A::Int], },
        InsDef { opcode: 328, alt_names: vec!["anmAlphaT"], arg_format: vec![A::Int, A::Int, A::Int, A::Int], },
        InsDef { opcode: 329, alt_names: vec!["anmScale"], arg_format: vec![A::Int, A::Float, A::Float], },
        InsDef { opcode: 330, alt_names: vec!["anmScaleT"], arg_format: vec![A::Int, A::Int, A::Int, A::Float, A::Float], },
        InsDef { opcode: 331, alt_names: vec!["anmAlpha2"], arg_format: vec![A::Int, A::Int], },
        InsDef { opcode: 332, alt_names: vec!["anmAlpha2T"], arg_format: vec![A::Int, A::Int, A::Int, A::Int], },
        InsDef { opcode: 333, alt_names: vec!["anmPosT"], arg_format: vec![A::Int, A::Int, A::Int, A::Float, A::Float], },
        InsDef { opcode: 334, alt_names: vec!["anm334"], arg_format: vec![A::Int], },
        InsDef { opcode: 335, alt_names: vec!["anmScale2"], arg_format: vec![A::Int, A::Float, A::Float], },
        InsDef { opcode: 336, alt_names: vec!["anmLayer"], arg_format: vec![A::Int, A::Int], },
        InsDef { opcode: 337, alt_names: vec!["anmBM_16_anmPlayPos"], arg_format: vec![A::Int, A::Int], },
        InsDef { opcode: 338, alt_names: vec!["anmPlayPos"], arg_format: vec![A::Int, A::Int, A::Float, A::Float, A::Float], },
        InsDef { opcode: 339, alt_names: vec!["anm339"], arg_format: vec![A::Int, A::Int, A::Int], },
        InsDef { opcode: 340, alt_names: vec!["enmDelete"], arg_format: vec![A::Int], },

        InsDef { opcode: 400, alt_names: vec!["movePos"], arg_format: vec![A::Float, A::Float] },
        InsDef { opcode: 401, alt_names: vec!["movePosTime"], arg_format: vec![A::Int, A::Int, A::Float, A::Float] },
        InsDef { opcode: 402, alt_names: vec!["movePosRel"], arg_format: vec![A::Float, A::Float] },
        InsDef { opcode: 403, alt_names: vec!["movePosRelTime"], arg_format: vec![A::Int, A::Int, A::Float, A::Float] },
        InsDef { opcode: 404, alt_names: vec!["moveVel"], arg_format: vec![A::Float, A::Float] },
        InsDef { opcode: 405, alt_names: vec!["moveVelTime"], arg_format: vec![A::Int, A::Int, A::Float, A::Float] },
        InsDef { opcode: 406, alt_names: vec!["moveVelRel"], arg_format: vec![A::Float, A::Float] },
        InsDef { opcode: 407, alt_names: vec!["moveVelRelTime"], arg_format: vec![A::Int, A::Int, A::Float, A::Float] },
        InsDef { opcode: 408, alt_names: vec!["moveCirc"], arg_format: vec![A::Float, A::Float, A::Float, A::Float] },
        InsDef { opcode: 409, alt_names: vec!["moveCircTime"], arg_format: vec![A::Int, A::Int, A::Float, A::Float, A::Float] },
        InsDef { opcode: 410, alt_names: vec!["moveCircRel"], arg_format: vec![A::Float, A::Float, A::Float, A::Float] },
        InsDef { opcode: 411, alt_names: vec!["moveCircRelTime"], arg_format: vec![A::Int, A::Int, A::Float, A::Float, A::Float] },
        InsDef { opcode: 412, alt_names: vec!["moveRand"], arg_format: vec![A::Int, A::Int, A::Float] },
        InsDef { opcode: 413, alt_names: vec!["moveRandRel"], arg_format: vec![A::Int, A::Int, A::Float] },
        InsDef { opcode: 414, alt_names: vec!["moveBoss"], arg_format: vec![] },
        InsDef { opcode: 415, alt_names: vec!["moveBossRel"], arg_format: vec![] },
        InsDef { opcode: 416, alt_names: vec!["movePos3d"], arg_format: vec![A::Float, A::Float, A::Float] },
        InsDef { opcode: 417, alt_names: vec!["movePos3dRel"], arg_format: vec![A::Float, A::Float, A::Float] },
        InsDef { opcode: 418, alt_names: vec!["moveAdd"], arg_format: vec![A::Float, A::Float] },
        InsDef { opcode: 419, alt_names: vec!["moveAddRel"], arg_format: vec![A::Float, A::Float] },
        InsDef { opcode: 420, alt_names: vec!["moveEll"], arg_format: vec![A::Float, A::Float, A::Float, A::Float, A::Float, A::Float] },
        InsDef { opcode: 421, alt_names: vec!["moveEllTime"], arg_format: vec![A::Int, A::Int, A::Float, A::Float, A::Float, A::Float, A::Float] },
        InsDef { opcode: 422, alt_names: vec!["moveEllRel"], arg_format: vec![A::Float, A::Float, A::Float, A::Float, A::Float, A::Float] },
        InsDef { opcode: 423, alt_names: vec!["moveEllRelTime"], arg_format: vec![A::Int, A::Int, A::Float, A::Float, A::Float, A::Float, A::Float] },
        InsDef { opcode: 424, alt_names: vec!["moveMirror"], arg_format: vec![A::Int] },
        InsDef { opcode: 425, alt_names: vec!["moveBezier"], arg_format: vec![A::Int, A::Float, A::Float, A::Float, A::Float, A::Float, A::Float] },
        InsDef { opcode: 426, alt_names: vec!["moveBezierRel"], arg_format: vec![A::Int, A::Float, A::Float, A::Float, A::Float, A::Float, A::Float] },
        InsDef { opcode: 427, alt_names: vec!["moveReset"], arg_format: vec![] },
        InsDef { opcode: 428, alt_names: vec!["moveVelNM"], arg_format: vec![A::Float, A::Float] },
        InsDef { opcode: 429, alt_names: vec!["moveVelTimeNM"], arg_format: vec![A::Int, A::Int, A::Float, A::Float] },
        InsDef { opcode: 430, alt_names: vec!["moveVelRelNM"], arg_format: vec![A::Float, A::Float] },
        InsDef { opcode: 431, alt_names: vec!["moveVelRelTimeNM"], arg_format: vec![A::Int, A::Int, A::Float, A::Float] },
        InsDef { opcode: 432, alt_names: vec!["moveEnm"], arg_format: vec![A::Int] },
        InsDef { opcode: 433, alt_names: vec!["moveEnmRel"], arg_format: vec![A::Int] },
        InsDef { opcode: 434, alt_names: vec!["moveCurve"], arg_format: vec![A::Int, A::Int, A::Int, A::Float, A::Float] },
        InsDef { opcode: 435, alt_names: vec!["moveCurveRel"], arg_format: vec![A::Int, A::Int, A::Int, A::Float, A::Float] },
        InsDef { opcode: 436, alt_names: vec!["moveAddTime"], arg_format: vec![A::Int, A::Int, A::Float, A::Float] },
        InsDef { opcode: 437, alt_names: vec!["moveAddRelTime"], arg_format: vec![A::Int, A::Int, A::Float, A::Float] },
        InsDef { opcode: 438, alt_names: vec!["moveCurveAdd"], arg_format: vec![A::Int, A::Int, A::Int, A::Float, A::Float] },
        InsDef { opcode: 439, alt_names: vec!["moveCurveAddRel"], arg_format: vec![A::Int, A::Int, A::Int, A::Float, A::Float] },
        InsDef { opcode: 440, alt_names: vec!["moveAngle"], arg_format: vec![A::Float] },
        InsDef { opcode: 441, alt_names: vec!["moveAngleTime"], arg_format: vec![A::Int, A::Int, A::Float] },
        InsDef { opcode: 442, alt_names: vec!["moveAngleRel"], arg_format: vec![A::Float] },
        InsDef { opcode: 443, alt_names: vec!["moveAngleRelTime"], arg_format: vec![A::Int, A::Int, A::Float] },
        InsDef { opcode: 444, alt_names: vec!["moveSpeed"], arg_format: vec![A::Float] },
        InsDef { opcode: 445, alt_names: vec!["moveSpeedTime"], arg_format: vec![A::Int, A::Int, A::Float] },
        InsDef { opcode: 446, alt_names: vec!["moveSpeedRel"], arg_format: vec![A::Float], },
        InsDef { opcode: 447, alt_names: vec!["moveSpeedRelTime"], arg_format: vec![A::Int, A::Int, A::Float] },

        InsDef { opcode: 500, alt_names: vec!["setHurtbox"], arg_format: vec![A::Float, A::Float] },
        InsDef { opcode: 501, alt_names: vec!["setHitbox"], arg_format: vec![A::Float, A::Float] },
        InsDef { opcode: 502, alt_names: vec!["flagSet"], arg_format: vec![A::Int] },
        InsDef { opcode: 503, alt_names: vec!["flagClear"], arg_format: vec![A::Int] },
        InsDef { opcode: 504, alt_names: vec!["moveLimit"], arg_format: vec![A::Float, A::Float, A::Float, A::Float] },
        InsDef { opcode: 505, alt_names: vec!["moveLimitReset"], arg_format: vec![] },
        InsDef { opcode: 506, alt_names: vec!["dropClear"], arg_format: vec![] },
        InsDef { opcode: 507, alt_names: vec!["dropExtra"], arg_format: vec![A::Int, A::Int] },
        InsDef { opcode: 508, alt_names: vec!["dropArea"], arg_format: vec![A::Float, A::Float] },
        InsDef { opcode: 509, alt_names: vec!["dropItems"], arg_format: vec![] },
        InsDef { opcode: 510, alt_names: vec!["dropMain"], arg_format: vec![A::Int] },
        InsDef { opcode: 511, alt_names: vec!["lifeSet"], arg_format: vec![A::Int] },
        InsDef { opcode: 512, alt_names: vec!["setBoss"], arg_format: vec![A::Int] },
        InsDef { opcode: 513, alt_names: vec!["timerReset"], arg_format: vec![] },
        InsDef { opcode: 514, alt_names: vec!["setInterrupt"], arg_format: vec![A::Int, A::Int, A::Int, A::Str]},
        InsDef { opcode: 515, alt_names: vec!["setInvuln"], arg_format: vec![A::Int]},
        InsDef { opcode: 516, alt_names: vec!["playSound"], arg_format: vec![A::Int]},
        InsDef { opcode: 517, alt_names: vec!["setScreenShake"], arg_format: vec![A::Int, A::Int, A::Int]},
        InsDef { opcode: 518, alt_names: vec!["dialogueRead"], arg_format: vec![A::Int]},
        InsDef { opcode: 519, alt_names: vec!["dialogueWait"], arg_format: vec![]},
        InsDef { opcode: 520, alt_names: vec!["bossWait"], arg_format: vec![]},
        InsDef { opcode: 521, alt_names: vec!["setTimeout"], arg_format: vec![A::Int, A::Str]},
        InsDef { opcode: 522, alt_names: vec!["spellEx"], arg_format: vec![A::Int, A::Int, A::Int, A::Str]}, // string needs to be encoded
        InsDef { opcode: 523, alt_names: vec!["spellEnd"], arg_format: vec![]},
        InsDef { opcode: 524, alt_names: vec!["setChapter"], arg_format: vec![A::Int]},
        InsDef { opcode: 525, alt_names: vec!["enmKillAll"], arg_format: vec![]},
        InsDef { opcode: 526, alt_names: vec!["etProtectRange"], arg_format: vec![A::Float]},
        InsDef { opcode: 527, alt_names: vec!["lifeMarker"], arg_format: vec![A::Int, A::Float, A::Int]},
        InsDef { opcode: 528, alt_names: vec!["spellUnused"], arg_format: vec![A::Int, A::Int, A::Int, A::Str]}, // string needs to be encoded
        InsDef { opcode: 529, alt_names: vec!["rankF3"], arg_format: vec![A::FloatRef, A::Float, A::Float, A::Float]},
        InsDef { opcode: 530, alt_names: vec!["rankF5"], arg_format: vec![A::FloatRef, A::Float, A::Float, A::Float, A::Float, A::Float]},
        InsDef { opcode: 531, alt_names: vec!["rankF2"], arg_format: vec![A::FloatRef, A::Float, A::Float]},
        InsDef { opcode: 532, alt_names: vec!["rankI3"], arg_format: vec![A::IntRef, A::Int, A::Int, A::Int]},
        InsDef { opcode: 533, alt_names: vec!["rankI5"], arg_format: vec![A::IntRef, A::Int, A::Int, A::Int, A::Int, A::Int]},
        InsDef { opcode: 534, alt_names: vec!["rankI2"], arg_format: vec![A::IntRef, A::Int, A::Int]},
        InsDef { opcode: 535, alt_names: vec!["diffI"], arg_format: vec![A::IntRef, A::Int, A::Int, A::Int, A::Int]},
        InsDef { opcode: 536, alt_names: vec!["diffF"], arg_format: vec![A::FloatRef, A::Float, A::Float, A::Float, A::Float]},
        InsDef { opcode: 537, alt_names: vec!["spell"], arg_format: vec![A::Int, A::Int, A::Int, A::Str]}, // string needs to be encoded
        InsDef { opcode: 538, alt_names: vec!["spell2"], arg_format: vec![A::Int, A::Int, A::Int, A::Str]}, // string needs to be encoded
        InsDef { opcode: 539, alt_names: vec!["spell3"], arg_format: vec![A::Int, A::Int, A::Int, A::Str]}, // string needs to be encoded
        InsDef { opcode: 540, alt_names: vec!["stars"], arg_format: vec![A::Int]},
        InsDef { opcode: 541, alt_names: vec!["noHbDur"], arg_format: vec![A::Int]},
        InsDef { opcode: 542, alt_names: vec!["spellTimeout"], arg_format: vec![]},
        InsDef { opcode: 543, alt_names: vec!["unknown543"], arg_format: vec![]},
        InsDef { opcode: 544, alt_names: vec!["unknown544"], arg_format: vec![A::Int]},
        InsDef { opcode: 545, alt_names: vec!["laserCancel"], arg_format: vec![]},
        InsDef { opcode: 546, alt_names: vec!["bombShield"], arg_format: vec![A::Int, A::Int]},
        InsDef { opcode: 547, alt_names: vec!["gameSpeed"], arg_format: vec![A::Float]},
        InsDef { opcode: 548, alt_names: vec!["diffWait"], arg_format: vec![A::Int, A::Int, A::Int, A::Int]},
        InsDef { opcode: 549, alt_names: vec!["unknown549"], arg_format: vec![A::Int]},
        InsDef { opcode: 550, alt_names: vec!["unknown550"], arg_format: vec![A::Int]},
        InsDef { opcode: 551, alt_names: vec!["unknown551"], arg_format: vec![A::Int]},
        InsDef { opcode: 552, alt_names: vec!["zIndex"], arg_format: vec![A::Int]},
        InsDef { opcode: 553, alt_names: vec!["hitSound"], arg_format: vec![A::Int]},
        InsDef { opcode: 554, alt_names: vec!["logo"], arg_format: vec![]},
        InsDef { opcode: 555, alt_names: vec!["enmAlive"], arg_format: vec![A::IntRef, A::Int]},
        InsDef { opcode: 556, alt_names: vec!["setDeath"], arg_format: vec![A::Str]},
        InsDef { opcode: 557, alt_names: vec!["fogTime"], arg_format: vec![A::Int, A::Int, A::Int, A::Float, A::Float]},
        InsDef { opcode: 558, alt_names: vec!["flagMirror"], arg_format: vec![A::Int]},
        InsDef { opcode: 559, alt_names: vec!["enmLimit"], arg_format: vec![A::Int]},
        InsDef { opcode: 560, alt_names: vec!["setBounceRect"], arg_format: vec![A::Float, A::Float]},
        InsDef { opcode: 561, alt_names: vec!["die"], arg_format: vec![]},
        InsDef { opcode: 562, alt_names: vec!["dropItemsSp"], arg_format: vec![]},
        InsDef { opcode: 563, alt_names: vec!["hbRect"], arg_format: vec![A::Int]},
        InsDef { opcode: 564, alt_names: vec!["hitboxRotate"], arg_format: vec![A::Float]},
        InsDef { opcode: 565, alt_names: vec!["bombInv"], arg_format: vec![A::Float]},
        InsDef { opcode: 566, alt_names: vec!["unknown566"], arg_format: vec![]},
        InsDef { opcode: 567, alt_names: vec!["unknown567"], arg_format: vec![A::Int]},
        InsDef { opcode: 568, alt_names: vec!["spellMode"], arg_format: vec![A::Int]},
        InsDef { opcode: 569, alt_names: vec!["unknown569"], arg_format: vec![A::Int]},
        InsDef { opcode: 570, alt_names: vec!["unknown570"], arg_format: vec![]},
        InsDef { opcode: 571, alt_names: vec!["unknown571"], arg_format: vec![]},
        InsDef { opcode: 572, alt_names: vec!["lifeNow"], arg_format: vec![A::Int]},

        InsDef { opcode: 600, alt_names: vec!["etNew"], arg_format: vec![A::Int]},
        InsDef { opcode: 601, alt_names: vec!["etOn"], arg_format: vec![A::Int]},
        InsDef { opcode: 602, alt_names: vec!["etSprite"], arg_format: vec![A::Int, A::Int, A::Int]},
        InsDef { opcode: 603, alt_names: vec!["etOffset"], arg_format: vec![A::Int, A::Float, A::Float]},
        InsDef { opcode: 604, alt_names: vec!["etAngle"], arg_format: vec![A::Int, A::Float, A::Float]},
        InsDef { opcode: 605, alt_names: vec!["etSpeed"], arg_format: vec![A::Int, A::Float, A::Float]},
        InsDef { opcode: 606, alt_names: vec!["etCount"], arg_format: vec![A::Int, A::Int, A::Int]},
        InsDef { opcode: 607, alt_names: vec!["etAim"], arg_format: vec![A::Int, A::Int]},
        InsDef { opcode: 608, alt_names: vec!["etSound"], arg_format: vec![A::Int, A::Int, A::Int]},
        InsDef { opcode: 609, alt_names: vec!["etExSet"], arg_format: vec![A::Int, A::Int, A::Int, A::Int, A::Int, A::Int, A::Float, A::Float]},
        InsDef { opcode: 610, alt_names: vec!["etExSet2"], arg_format: vec![A::Int, A::Int, A::Int, A::Int, A::Int, A::Int, A::Int, A::Int, A::Float, A::Float, A::Float, A::Float]},
        InsDef { opcode: 611, alt_names: vec!["etEx"], arg_format: vec![A::Int, A::Int, A::Int, A::Int, A::Int, A::Float, A::Float]},
        InsDef { opcode: 612, alt_names: vec!["etEx2"], arg_format: vec![A::Int, A::Int, A::Int, A::Int, A::Int, A::Int, A::Int, A::Float, A::Float, A::Float, A::Float]},
        InsDef { opcode: 613, alt_names: vec!["etClearAll"], arg_format: vec![]},
        InsDef { opcode: 614, alt_names: vec!["etCopy"], arg_format: vec![A::Int, A::Int]},
        InsDef { opcode: 615, alt_names: vec!["etCancel"], arg_format: vec![A::Float]},
        InsDef { opcode: 616, alt_names: vec!["etClear"], arg_format: vec![A::Float]},
        InsDef { opcode: 617, alt_names: vec!["etSpeedR3"], arg_format: vec![A::Int, A::Float, A::Float, A::Float, A::Float, A::Float, A::Float]},
        InsDef { opcode: 618, alt_names: vec!["etSpeedR5"], arg_format: vec![A::Int, A::Float, A::Float, A::Float, A::Float, A::Float, A::Float, A::Float, A::Float, A::Float, A::Float]},
        InsDef { opcode: 619, alt_names: vec!["etSpeedR2"], arg_format: vec![A::Int, A::Float, A::Float, A::Float, A::Float]},
        InsDef { opcode: 620, alt_names: vec!["etCountR3"], arg_format: vec![A::Int, A::Int, A::Int, A::Int, A::Int, A::Int, A::Int]},
        InsDef { opcode: 621, alt_names: vec!["etCountR5"], arg_format: vec![A::Int, A::Int, A::Int, A::Int, A::Int, A::Int, A::Int, A::Int, A::Int, A::Int, A::Int]},
        InsDef { opcode: 622, alt_names: vec!["etCountR2"], arg_format: vec![A::Int, A::Int, A::Int, A::Int, A::Int]},
        InsDef { opcode: 623, alt_names: vec!["angleToPlayer"], arg_format: vec![A::FloatRef, A::Float, A::Float]},
        InsDef { opcode: 624, alt_names: vec!["etSpeedD"], arg_format: vec![A::Int, A::Float, A::Float, A::Float, A::Float, A::Float, A::Float, A::Float, A::Float]},
        InsDef { opcode: 625, alt_names: vec!["etCountD"], arg_format: vec![A::Int, A::Int, A::Int, A::Int, A::Int, A::Int, A::Int, A::Int, A::Int]},
        InsDef { opcode: 626, alt_names: vec!["etOffsetRad"], arg_format: vec![A::Int, A::Float, A::Float]},
        InsDef { opcode: 627, alt_names: vec!["etDist"], arg_format: vec![A::Int, A::Float]},
        InsDef { opcode: 628, alt_names: vec!["etOrigin"], arg_format: vec![A::Int, A::Float, A::Float]},
        InsDef { opcode: 629, alt_names: vec!["fog"], arg_format: vec![A::Float, A::Int]},
        InsDef { opcode: 630, alt_names: vec!["callStd"], arg_format: vec![A::Int]},
        InsDef { opcode: 631, alt_names: vec!["lifeHide"], arg_format: vec![A::Int]},
        InsDef { opcode: 632, alt_names: vec!["funcSet"], arg_format: vec![A::Int]},
        InsDef { opcode: 633, alt_names: vec!["flagExtDmg"], arg_format: vec![A::Int]},
        InsDef { opcode: 634, alt_names: vec!["setHitboxFunc"], arg_format: vec![A::Int]},
        InsDef { opcode: 635, alt_names: vec!["etCancelAsBomb"], arg_format: vec![A::Float]},
        InsDef { opcode: 636, alt_names: vec!["etClearAsBomb"], arg_format: vec![A::Float]},
        InsDef { opcode: 637, alt_names: vec!["funcCall"], arg_format: vec![A::Int]},
        InsDef { opcode: 638, alt_names: vec!["scoreAdd"], arg_format: vec![A::Int]},
        InsDef { opcode: 639, alt_names: vec!["funcSet2"], arg_format: vec![A::Int]},
        InsDef { opcode: 640, alt_names: vec!["etExSub"], arg_format: vec![A::Int, A::Int, A::Str]},
        InsDef { opcode: 641, alt_names: vec!["etExSubtract"], arg_format: vec![A::Int]},

        InsDef { opcode: 700, alt_names: vec!["laserNew"], arg_format: vec![A::Int, A::Float, A::Float, A::Float, A::Float]},
        InsDef { opcode: 701, alt_names: vec!["laserTiming"], arg_format: vec![A::Int, A::Int, A::Int, A::Int, A::Int, A::Int]},
        InsDef { opcode: 702, alt_names: vec!["laserOn"], arg_format: vec![A::Int]},
        InsDef { opcode: 703, alt_names: vec!["laserStOn"], arg_format: vec![A::Int, A::Int]},
        InsDef { opcode: 704, alt_names: vec!["laserOffset"], arg_format: vec![A::Int, A::Float, A::Float]},
        InsDef { opcode: 705, alt_names: vec!["laserTrajectory"], arg_format: vec![A::Int, A::Float, A::Float]},
        InsDef { opcode: 706, alt_names: vec!["laserStLength"], arg_format: vec![A::Int, A::Float]},
        InsDef { opcode: 707, alt_names: vec!["laserStWidth"], arg_format: vec![A::Int, A::Float]},
        InsDef { opcode: 708, alt_names: vec!["laserStAngle"], arg_format: vec![A::Int, A::Float]},
        InsDef { opcode: 709, alt_names: vec!["laserStRotation"], arg_format: vec![A::Int, A::Float]},
        InsDef { opcode: 710, alt_names: vec!["laserStEnd"], arg_format: vec![A::Int]},
        InsDef { opcode: 711, alt_names: vec!["laserCuOn"], arg_format: vec![A::Int]},
        InsDef { opcode: 712, alt_names: vec!["etCancelRect"], arg_format: vec![A::Float, A::Float]},
        InsDef { opcode: 713, alt_names: vec!["LaserBeOn"], arg_format: vec![A::Int, A::Int]},
        InsDef { opcode: 714, alt_names: vec!["LaserBeCall"], arg_format: vec![A::Int, A::Int]},

        InsDef { opcode: 800, alt_names: vec!["enmCall"], arg_format: vec![A::Int, A::Str]},
        InsDef { opcode: 801, alt_names: vec!["enmPos"], arg_format: vec![A::FloatRef, A::FloatRef, A::Int]},
        InsDef { opcode: 802, alt_names: vec!["broadcastInt"], arg_format: vec![A::Int]},
    ];
    v
    };
}
