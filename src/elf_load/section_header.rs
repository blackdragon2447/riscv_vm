use super::data::{SectionFlags, SectionType};

struct RawSectionHeader {
    // 0x00 : 4
    name: [u8; 4],
    // 0x04 : 4
    sec_type: [u8; 4],
    // 0x08 : 8
    flags: [u8; 8],
    // 0x10 : 8
    sec_addr: [u8; 8],
    // 0x18 : 8
    sec_offset: [u8; 8],
    // 0x20 : 8
    sec_size: [u8; 8],
    // 0x28 : 4
    sec_link: [u8; 4],
    // 0x2C : 4
    sec_info: [u8; 4],
    // 0x30 : 8
    sec_align: [u8; 8],
    // 0x38 : 8
    sec_entry_size: [u8; 8],
}

pub enum SectionName {
    Offset(u32),
    String(u32, String),
}

pub struct SectionHeader {
    name: SectionName,
    sec_type: SectionType,
    flags: SectionFlags,
    sec_addr: u64,
    sec_offset: u64,
    sec_size: u64,
    sec_link: u32,
    sec_info: u32,
    sec_align: u64,
    sec_entry_size: Option<u64>,
}
