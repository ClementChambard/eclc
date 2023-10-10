mod header;
mod instr;
mod sub;

pub use header::generate;
pub use instr::gen_inscall;
pub use instr::gen_instr;
pub use instr::resolve_ins_opcode;
pub use sub::gen_sub;
