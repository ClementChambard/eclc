use crate::{
    ast::{Expr, Instr, TimeLabelKind},
    error::report_error_ext,
};

struct InstrCallCode {
    time: u32,
    opcode: u16,
    size: u16,
    param_mask: u16,
    rank_mask: u8,
    param_count: u8,
    cur_stack_ref: u32,
}

impl InstrCallCode {
    fn to_ne_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(std::mem::size_of::<InstrCallCode>());

        bytes.extend_from_slice(&self.time.to_ne_bytes());
        bytes.extend_from_slice(&self.opcode.to_ne_bytes());
        bytes.extend_from_slice(&self.size.to_ne_bytes());
        bytes.extend_from_slice(&self.param_mask.to_ne_bytes());
        bytes.extend_from_slice(&self.rank_mask.to_ne_bytes());
        bytes.extend_from_slice(&self.param_count.to_ne_bytes());
        bytes.extend_from_slice(&self.cur_stack_ref.to_ne_bytes());

        bytes
    }
}

pub fn resolve_ins_opcode(ins_name: &str) -> u16 {
    ins_name.strip_prefix("ins_").unwrap().parse().unwrap()
}

fn get_arg_size(args: &[CallArg]) -> u16 {
    args.iter().map(|a| a.size()).sum()
}

fn get_param_mask(args: &[Expr]) -> u16 {
    let mut mask = 0u16;
    for a in args.iter().rev() {
        mask <<= 1;
        if a.is_var() {
            mask |= 1;
        }
    }
    mask
}

enum CallArg {
    Str(String),
    Int(i32),
    Float(f32),
    Vararg(Vec<CallArg>),
    Err,
}

impl CallArg {
    fn size(&self) -> u16 {
        match self {
            Self::Str(s) => {
                let mut strlen = s.len() as u16 + 1;
                let mod4 = strlen % 4;
                if mod4 != 0 {
                    strlen = strlen - mod4 + 4;
                }
                strlen + 4
            }
            Self::Vararg(va) => (va.len() * 8) as u16,
            _ => 4,
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        match self {
            Self::Str(s) => {
                let mut bytes = Vec::new();
                let strlen = s.len() as u32 + 1;
                let padding = self.size() as u32 - 4 - strlen;
                bytes.extend_from_slice(&(strlen + padding).to_ne_bytes());
                // should encode to Shift-JIS
                bytes.extend(s.bytes());
                bytes.push(0u8);
                bytes.extend(vec![0u8; padding as usize]);
                bytes
            }
            Self::Float(f) => f.to_ne_bytes().to_vec(),
            Self::Int(i) => i.to_ne_bytes().to_vec(),
            Self::Vararg(va) => {
                let mut bytes = Vec::new();
                for v in va {
                    match v {
                        Self::Float(f) => {
                            bytes.extend(vec![b'f', b'f', 0u8, 0u8]);
                            bytes.extend_from_slice(&f.to_ne_bytes());
                        }
                        Self::Int(i) => {
                            bytes.extend(vec![b'i', b'i', 0u8, 0u8]);
                            bytes.extend_from_slice(&i.to_ne_bytes());
                        }
                        _ => {}
                    }
                }
                bytes
            }
            Self::Err => panic!(),
        }
    }
}

impl From<&Expr> for CallArg {
    fn from(value: &Expr) -> Self {
        match value {
            Expr::VarInt(i) => Self::Int(*i.val()),
            Expr::VarFloat(f) => Self::Float(*f.val()),
            Expr::Int(i) => Self::Int(*i.val()),
            Expr::Float(f) => Self::Float(*f.val()),
            Expr::Str(s) => Self::Str(s.val().clone()),
            Expr::Vararg(va) => Self::Vararg(va.clone().iter().map(CallArg::from).collect()),
            Expr::Id(id) => {
                report_error_ext(
                    id.loc(),
                    &format!("unresolved identifier `{}`", id.val()),
                    "unresolved identifier",
                );
                Self::Err
            }
            _ => {
                panic!()
            }
        }
    }
}

pub fn gen_instr(i: &Instr, time_now: &mut u32, rank_now: &mut u8) -> Vec<u8> {
    match i {
        Instr::Call(name, args) => gen_inscall(name.val(), args, *time_now, *rank_now),
        Instr::Bloc(insts) => {
            let mut bytes = vec![];
            for i in insts {
                bytes.extend(gen_instr(i, time_now, rank_now));
            }
            bytes
        }
        Instr::Label(_) => vec![], // no code to generate for label
        Instr::TimeLabel(t, k) => {
            match k {
                TimeLabelKind::Set => *time_now = *t.val() as u32,
                TimeLabelKind::Add => *time_now += *t.val() as u32,
                TimeLabelKind::Sub => *time_now -= *t.val() as u32,
            };
            vec![]
        }
        Instr::RankLabel(r) => {
            *rank_now = *r.val();
            vec![]
        }
        _ => panic!("Can't generate instruction {:?}", i),
    }
}

fn get_stack_ref(args: &[Expr]) -> u32 {
    let mut cnt = 0;
    for a in args {
        match a {
            Expr::VarInt(i) => {
                if *(i.val()) < 0 && *(i.val()) > -200 {
                    cnt += 1;
                }
            }
            Expr::VarFloat(f) => {
                if *(f.val()) < 0. && *(f.val()) > -200. {
                    cnt += 1;
                }
            }
            _ => {}
        }
    }
    cnt
}

fn get_param_count(args: &[Expr]) -> u8 {
    let mut cnt = 0;
    for a in args {
        if let Expr::Vararg(va) = a {
            cnt += va.len() as u8;
        } else {
            cnt += 1;
        }
    }
    cnt
}

pub fn gen_inscall(name: &str, args: &[Expr], time_now: u32, rank_now: u8) -> Vec<u8> {
    let callargs: Vec<CallArg> = args.iter().map(CallArg::from).collect();
    let mut code = InstrCallCode {
        time: time_now,
        opcode: resolve_ins_opcode(name),
        size: get_arg_size(&callargs) + 16,
        param_mask: get_param_mask(args),
        rank_mask: rank_now,
        param_count: get_param_count(args),
        cur_stack_ref: get_stack_ref(args),
    }
    .to_ne_bytes();
    for a in callargs {
        code.extend_from_slice(&a.to_bytes());
    }
    code
}
