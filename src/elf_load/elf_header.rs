use std::fmt::Debug;

use crate::memory::Address;

use super::{
    data::{AbiType, Bitness, Endianess, ObjectType, ASI},
    BitRanges, ElfHeaderParseError, ElfParseError,
};

#[derive(Debug)]
struct RawElfHeader {
    /// 0x00 : 4
    magic: [u8; 4],
    /// 0x04 : 1
    bitness: [u8; 1],
    /// 0x05 : 1
    endianess: [u8; 1],
    /// 0x06 : 1
    version: [u8; 1],
    /// 0x07 : 1
    abi: [u8; 1],
    /// 0x08 : 1
    abi_version: [u8; 1],
    /// 0x09 : 7 : padding
    /// 0x10 : 2
    obj_type: [u8; 2],
    /// 0x12 : 2
    arch: [u8; 2],
    /// 0x14 :  4
    version_orig: [u8; 4],
    /// 0x18 : 8
    entry: [u8; 8],
    /// 0x20 : 8
    p_header: [u8; 8],
    /// 0x28 : 8
    s_header: [u8; 8],
    /// 0x30 : 4
    flags: [u8; 4],
    /// 0x34 : 2
    header_size: [u8; 2],
    /// 0x36 : 2
    p_header_size: [u8; 2],
    /// 0x38 : 2
    p_header_ecount: [u8; 2],
    /// 0x3A : 2
    s_header_size: [u8; 2],
    /// 0x3C : 2
    s_header_ecount: [u8; 2],
    /// 0x3E : 2
    s_header_name_entry: [u8; 2],
}

impl RawElfHeader {
    /// Bytes may be longer than the header, header is assumed to be at 0x00
    fn from_bytes(bytes: &Vec<u8>) -> RawElfHeader {
        Self {
            magic: bytes.get_bytes_copy(0x00),
            bitness: bytes.get_bytes_copy(0x04),
            endianess: bytes.get_bytes_copy(0x05),
            version: bytes.get_bytes_copy(0x06),
            abi: bytes.get_bytes_copy(0x07),
            abi_version: bytes.get_bytes_copy(0x08),
            obj_type: bytes.get_bytes_copy(0x10),
            arch: bytes.get_bytes_copy(0x12),
            version_orig: bytes.get_bytes_copy(0x14),
            entry: bytes.get_bytes_copy(0x18),
            p_header: bytes.get_bytes_copy(0x20),
            s_header: bytes.get_bytes_copy(0x28),
            flags: bytes.get_bytes_copy(0x30),
            header_size: bytes.get_bytes_copy(0x34),
            p_header_size: bytes.get_bytes_copy(0x36),
            p_header_ecount: bytes.get_bytes_copy(0x38),
            s_header_size: bytes.get_bytes_copy(0x3A),
            s_header_ecount: bytes.get_bytes_copy(0x3C),
            s_header_name_entry: bytes.get_bytes_copy(0x3E),
        }
    }
}

#[derive(Debug)]
pub struct ElfHeader {
    pub bitness: Bitness,
    pub endianess: Endianess,
    pub abi_type: AbiType,
    pub abi_ver: u8,
    pub obj_type: ObjectType,
    pub arch: ASI,
    pub entry: Address,
    pub p_header: u64,
    pub s_header: u64,
    pub flags: u32,
    // header_size: u16,
    pub p_header_size: u16,
    pub p_header_ecount: u16,
    pub s_header_size: u16,
    pub s_header_ecount: u16,
    pub s_header_name_entry: u16,
}

impl ElfHeader {
    /// Bytes may be longer than the header, header is assumed to be at 0x00
    pub fn from_bytes(bytes: &Vec<u8>) -> Result<ElfHeader, ElfHeaderParseError> {
        let raw = RawElfHeader::from_bytes(&bytes);

        if raw.magic != [0x7F, 0x45, 0x4C, 0x46] {
            return Err(ElfHeaderParseError::InvalidMagic);
        }

        let bitness = match u8::from_le_bytes(raw.bitness) {
            1 => Bitness::B32,
            2 => Bitness::B64,
            i => return Err(ElfHeaderParseError::InvalidBitness(i)),
        };

        let endianess = Endianess::try_from(u8::from_le_bytes(raw.endianess))?;

        let version = u8::from_le_bytes(raw.version);
        if version != 1 {
            return Err(ElfHeaderParseError::InvalidVersion(version));
        }

        let abi = AbiType::try_from(u8::from_le_bytes(raw.abi))?;
        let abi_ver = u8::from_le_bytes(raw.abi_version);

        let obj_type = ObjectType::try_from(u16::from_le_bytes(raw.obj_type))?;

        let arch = ASI::try_from(u16::from_le_bytes(raw.arch))?;

        let version_orig = u32::from_le_bytes(raw.version_orig);
        if version_orig != 1 {
            return Err(ElfHeaderParseError::InvalidVersionOrig(version_orig));
        }

        let entry = u64::from_le_bytes(raw.entry).into();

        let p_header = u64::from_le_bytes(raw.p_header);
        let s_header = u64::from_le_bytes(raw.s_header);

        let flags = u32::from_le_bytes(raw.flags);

        let header_size = u16::from_le_bytes(raw.header_size);
        match bitness {
            Bitness::B32 => {
                if header_size != 52 {
                    return Err(ElfHeaderParseError::InvalidSize(header_size));
                }
            }
            Bitness::B64 => {
                if header_size != 64 {
                    return Err(ElfHeaderParseError::InvalidSize(header_size));
                }
            }
            Bitness::B128 => unreachable!(),
        }

        let p_header_size = u16::from_le_bytes(raw.p_header_size);
        let p_header_ecount = u16::from_le_bytes(raw.p_header_ecount);
        let s_header_size = u16::from_le_bytes(raw.s_header_size);
        let s_header_ecount = u16::from_le_bytes(raw.s_header_ecount);
        let s_header_name_entry = u16::from_le_bytes(raw.s_header_name_entry);

        Ok(Self {
            bitness,
            endianess,
            abi_type: abi,
            abi_ver,
            obj_type,
            arch,
            entry,
            p_header,
            s_header,
            flags,
            p_header_size,
            p_header_ecount,
            s_header_size,
            s_header_ecount,
            s_header_name_entry,
        })
    }
}
