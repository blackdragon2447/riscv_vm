use super::data::{AbiType, Architecture, Bitness, Endianess, ObjectType};

struct RawElfHeader {
    // 0x00 : 4
    magic: [u8; 4],
    // 0x04 : 1
    bitness: u8,
    // 0x05 : 1
    endianess: u8,
    // 0x06 : 1
    version: u8,
    // 0x87 : 1
    abi: u8,
    // 0x08 : 1
    abi_version: u8,
    // 0x09 : 7 : padding
    // 0x10 : 2
    obj_type: [u8; 2],
    // 0x12 : 2
    arch: [u8; 2],
    // 0x14 :  4
    version_orig: [u8; 4],
    // 0x18 : 8
    entry: [u8; 8],
    // 0x20 : 8
    p_header: [u8; 8],
    // 0x28 : 8
    s_header: [u8; 8],
    // 0x30 : 4
    flags: [u8; 4],
    // 0x34 : 2
    header_size: [u8; 2],
    // 0x36 : 2
    p_header_size: [u8; 2],
    // 0x38 : 2
    p_header_ecount: [u8; 2],
    // 0x3A : 2
    s_header_size: [u8; 2],
    // 0x3C : 2
    s_header_ecount: [u8; 2],
    // 0x3E : 2
    s_header_name_entry: [u8; 2],
}

struct ElfHeader {
    magic: [u8; 4],
    bitness: Bitness,
    endianess: Endianess,
    version: u8,
    abi_type: AbiType,
    abi_ver: Option<u8>,
    obj_type: ObjectType,
    arch: Architecture,
    entry: u64,
    p_header: u64,
    s_header: u64,
    flags: u64,
    header_size: u16,
    p_header_size: u16,
    p_header_ecount: u16,
    s_header_size: u16,
    s_header_ecount: u16,
    s_header_name_entry: u16,
}
