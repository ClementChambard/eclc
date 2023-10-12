use crate::ast::Ecl;

#[repr(C)]
struct EclCode {
    magic: [u8; 4],
    unknown1: u16,
    include_length: u16,
    include_offset: u32,
    zero1: [u8; 4],
    sub_count: u32,
    zero2: [u8; 16],
}

impl EclCode {
    fn new(include_length: u16, sub_count: u32) -> Self {
        Self {
            magic: [b'S', b'C', b'P', b'T'],
            unknown1: 1u16,
            include_length,
            include_offset: 36,
            zero1: [0, 0, 0, 0],
            sub_count,
            zero2: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut b = Vec::new();
        b.extend_from_slice(&self.magic);
        b.extend_from_slice(&self.unknown1.to_ne_bytes());
        b.extend_from_slice(&self.include_length.to_ne_bytes());
        b.extend_from_slice(&self.include_offset.to_ne_bytes());
        b.extend_from_slice(&self.zero1);
        b.extend_from_slice(&self.sub_count.to_ne_bytes());
        b.extend_from_slice(&self.zero2);
        b
    }
}

#[repr(C)]
struct IncListHeaderCode {
    magic: [u8; 4],
    count: u32,
}

impl IncListHeaderCode {
    fn new(typ: &str, count: usize) -> Self {
        let mut chrs = typ.chars();
        let magic: [u8; 4] = [
            chrs.next().unwrap() as u8,
            chrs.next().unwrap() as u8,
            chrs.next().unwrap() as u8,
            chrs.next().unwrap() as u8,
        ];
        Self {
            magic,
            count: count as u32,
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut data = self.magic.to_vec();
        data.extend_from_slice(&self.count.to_ne_bytes());
        data
    }
}

struct IncList<'a> {
    header: IncListHeaderCode,
    strings: &'a Vec<String>,
}

impl<'a> IncList<'a> {
    fn new(typ: &str, strings: &'a Vec<String>) -> Self {
        let header = IncListHeaderCode::new(typ, strings.len());
        Self { header, strings }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut b = self.header.to_bytes();
        for s in self.strings {
            b.extend(s.bytes());
            b.push(0);
        }
        let length = b.len();
        if length % 4 != 0 {
            let padding = 4 - (length % 4);
            b.extend(vec![0u8; padding]);
        }
        b
    }
}

pub fn generate(ecl: &Ecl) -> Vec<u8> {
    let anmi = IncList::new("ANIM", &ecl.anmi).to_bytes();
    let ecli = IncList::new("ECLI", &ecl.ecli).to_bytes();
    let include_size = anmi.len() + ecli.len();
    let header = EclCode::new(include_size as u16, ecl.subs.len() as u32);
    let mut bytes = header.to_bytes();
    bytes.extend(anmi);
    bytes.extend(ecli);

    let mut sub_names = Vec::new();
    for s in &ecl.subs {
        sub_names.extend(s.name.bytes());
        sub_names.push(0u8);
    }
    sub_names.extend(vec![0u8; 4 - (sub_names.len() % 4)]);

    let mut sub_offset = bytes.len() + 4 * ecl.subs.len() + sub_names.len();
    let mut sub_data = Vec::new();
    let mut sub_offsets = Vec::new();
    // offset table
    for s in &ecl.subs {
        sub_offsets.extend_from_slice(&(sub_offset as u32).to_ne_bytes());
        let sub_bytes = super::gen_sub(s);
        sub_offset += sub_bytes.len();
        sub_data.extend_from_slice(&sub_bytes);
    }
    bytes.extend(sub_offsets);
    bytes.extend(sub_names);
    bytes.extend(sub_data);
    bytes
}
