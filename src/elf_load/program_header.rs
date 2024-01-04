use super::data::{ProgramFlags, ProgramType};

struct RawProgramHeader {
    // 0x00 : 4
    segment_type: [u8; 4],
    // 0x04 : 4
    flags: [u8; 4],
    // 0x08 : 8
    seg_offset: [u8; 8],
    // 0x10 : 8
    seg_v_addr: [u8; 8],
    // 0x18 : 8
    seg_p_addr: [u8; 8],
    // 0x20 : 8
    seg_f_size: [u8; 8],
    // 0x28 : 8
    seg_m_size: [u8; 8],
    // 0x30 : 8
    align: [u8; 8],
}

struct ProgramHeader {
    program_type: ProgramType,
    flasgs: ProgramFlags,
    seg_offset: u64,
    seg_v_addr: u64,
    seg_p_addr: u64,
    seg_f_size: u64,
    seg_m_size: u64,
    align: u64,
}
