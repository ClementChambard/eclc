use crate::ast::Sub;

#[repr(C)]
struct SubHeaderCode {
    magic: [u8; 4],
    data_offset: u32,
    zeros: [u8; 8],
}

impl SubHeaderCode {
    fn new() -> Self {
        Self {
            magic: [b'E', b'C', b'L', b'H'],
            data_offset: 16,
            zeros: [0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8],
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = self.magic.to_vec();
        bytes.extend_from_slice(&self.data_offset.to_ne_bytes());
        bytes.extend_from_slice(&self.zeros);
        bytes
    }
}

pub fn gen_sub(sub: &Sub) -> Vec<u8> {
    let mut bytes = SubHeaderCode::new().to_bytes();
    let mut time = 0u32;
    let mut rank = 255u8;
    for i in &sub.instructions {
        bytes.extend(super::gen_instr(i, &mut time, &mut rank));
    }
    bytes
}
