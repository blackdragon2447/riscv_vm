use enumflags2::BitFlags;

use crate::{error::SectionHeaderParseError, Address};

use super::{
    data::{SectionFlags, SectionType},
    ByteRanges,
};

struct RawSectionHeader {
    /// 0x00 : 4
    name: [u8; 4],
    /// 0x04 : 4
    sec_type: [u8; 4],
    /// 0x08 : 8
    flags: [u8; 8],
    /// 0x10 : 8
    sec_addr: [u8; 8],
    /// 0x18 : 8
    sec_offset: [u8; 8],
    /// 0x20 : 8
    sec_size: [u8; 8],
    /// 0x28 : 4
    sec_link: [u8; 4],
    /// 0x2C : 4
    sec_info: [u8; 4],
    /// 0x30 : 8
    sec_align: [u8; 8],
    /// 0x38 : 8
    sec_entry_size: [u8; 8],
}

impl RawSectionHeader {
    fn from_bytes(bytes: &[u8]) -> RawSectionHeader {
        Self {
            name: bytes.get_bytes_copy(0x00),
            sec_type: bytes.get_bytes_copy(0x04),
            flags: bytes.get_bytes_copy(0x08),
            sec_addr: bytes.get_bytes_copy(0x10),
            sec_offset: bytes.get_bytes_copy(0x18),
            sec_size: bytes.get_bytes_copy(0x20),
            sec_link: bytes.get_bytes_copy(0x28),
            sec_info: bytes.get_bytes_copy(0x2C),
            sec_align: bytes.get_bytes_copy(0x30),
            sec_entry_size: bytes.get_bytes_copy(0x38),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SectionName {
    Offset(u32),
    String(u32, String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SectionHeader {
    pub name: SectionName,
    pub sec_type: SectionType,
    pub flags: BitFlags<SectionFlags>,
    pub sec_addr: Address,
    pub sec_offset: u64,
    pub sec_size: u64,
    pub sec_link: u32,
    pub sec_info: u32,
    pub sec_align: u64,
    pub sec_entry_size: Option<u64>,
}

impl SectionHeader {
    pub fn from_bytes(
        bytes: &Vec<u8>,
        offset: u64,
        size: u64,
    ) -> Result<SectionHeader, SectionHeaderParseError> {
        let raw = RawSectionHeader::from_bytes(bytes.get_bytes(offset, size));

        let name = SectionName::Offset(u32::from_le_bytes(raw.name));

        let sec_type = SectionType::try_from(u32::from_le_bytes(raw.sec_type))?;

        let flags: BitFlags<SectionFlags> = BitFlags::from_bits(u64::from_le_bytes(raw.flags))?;

        let sec_addr = u64::from_le_bytes(raw.sec_addr).into();
        let sec_offset = u64::from_le_bytes(raw.sec_offset);
        let sec_size = u64::from_le_bytes(raw.sec_size);
        let sec_link = u32::from_le_bytes(raw.sec_link);
        let sec_info = u32::from_le_bytes(raw.sec_info);
        let sec_align = u64::from_le_bytes(raw.sec_align);

        let sec_entry_size = match u64::from_le_bytes(raw.sec_entry_size) {
            0 => None,
            i => Some(i),
        };

        Ok(Self {
            name,
            sec_type,
            flags,
            sec_addr,
            sec_offset,
            sec_size,
            sec_link,
            sec_info,
            sec_align,
            sec_entry_size,
        })
    }
}
