use enumflags2::BitFlags;

use crate::{error::ProgramHeaderParseError, Address};

use super::{
    data::{ProgramFlags, ProgramType},
    ByteRanges,
};

struct RawProgramHeader {
    /// 0x00 : 4
    program_type: [u8; 4],
    /// 0x04 : 4
    flags: [u8; 4],
    /// 0x08 : 8
    seg_offset: [u8; 8],
    /// 0x10 : 8
    seg_v_addr: [u8; 8],
    /// 0x18 : 8
    seg_p_addr: [u8; 8],
    /// 0x20 : 8
    seg_f_size: [u8; 8],
    /// 0x28 : 8
    seg_m_size: [u8; 8],
    /// 0x30 : 8
    align: [u8; 8],
}

impl RawProgramHeader {
    fn from_bytes(bytes: &[u8]) -> RawProgramHeader {
        Self {
            program_type: bytes.get_bytes_copy(0x00),
            flags: bytes.get_bytes_copy(0x04),
            seg_offset: bytes.get_bytes_copy(0x08),
            seg_v_addr: bytes.get_bytes_copy(0x10),
            seg_p_addr: bytes.get_bytes_copy(0x18),
            seg_f_size: bytes.get_bytes_copy(0x20),
            seg_m_size: bytes.get_bytes_copy(0x28),
            align: bytes.get_bytes_copy(0x30),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ProgramHeader {
    pub program_type: ProgramType,
    pub flags: BitFlags<ProgramFlags>,
    pub seg_offset: u64,
    pub seg_v_addr: Address,
    pub seg_p_addr: Address,
    pub seg_f_size: Address,
    pub seg_m_size: Address,
    pub align: u64,
}

impl ProgramHeader {
    pub fn from_bytes(
        bytes: &Vec<u8>,
        offset: u64,
        size: u64,
    ) -> Result<ProgramHeader, ProgramHeaderParseError> {
        let raw = RawProgramHeader::from_bytes(bytes.get_bytes(offset, size));

        let program_type = ProgramType::try_from(u32::from_le_bytes(raw.program_type))?;

        let flags: BitFlags<ProgramFlags> = BitFlags::from_bits(u32::from_le_bytes(raw.flags))?;

        let seg_offset = u64::from_le_bytes(raw.seg_offset);

        let seg_v_addr = u64::from_le_bytes(raw.seg_v_addr).into();
        let seg_p_addr = u64::from_le_bytes(raw.seg_p_addr).into();

        let seg_f_size = u64::from_le_bytes(raw.seg_f_size).into();
        let seg_m_size = u64::from_le_bytes(raw.seg_m_size).into();

        let align = u64::from_le_bytes(raw.align);

        Ok(Self {
            program_type,
            flags,
            seg_offset,
            seg_v_addr,
            seg_p_addr,
            seg_f_size,
            seg_m_size,
            align,
        })
    }
}
