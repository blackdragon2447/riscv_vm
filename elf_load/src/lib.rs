use crate::{data::SectionType, section_header::SectionName};

use self::{program_header::ProgramHeader, section_header::SectionHeader};
use elf_header::ElfHeader;
use error::ElfParseError;
use std::{fmt::Debug, usize};

pub mod data;
pub mod elf_header;
pub mod error;
pub mod program_header;
pub mod section_header;
#[cfg(test)]
mod tests;

pub struct Elf {
    pub header: elf_header::ElfHeader,
    pub program_headers: Vec<ProgramHeader>,
    pub section_headers: Vec<SectionHeader>,
    pub bytes: Vec<u8>,
}

impl Elf {
    pub fn from_bytes(bytes: Vec<u8>) -> Result<Elf, ElfParseError> {
        let header = ElfHeader::from_bytes(&bytes)?;

        let mut p_headers = vec![];

        for i in 0..header.p_header_ecount {
            p_headers.push(ProgramHeader::from_bytes(
                &bytes,
                header.p_header + (i * header.p_header_size) as u64,
                header.p_header_size.into(),
            )?);
        }

        let mut s_headers = vec![];

        for i in 0..header.s_header_ecount {
            s_headers.push(SectionHeader::from_bytes(
                &bytes,
                header.s_header + (i * header.s_header_size) as u64,
                header.s_header_size.into(),
            )?);
        }

        Ok(Self {
            header,
            program_headers: p_headers,
            section_headers: s_headers,
            bytes,
        })
    }

    pub fn populate_secion_names(&mut self) -> Result<(), ElfParseError> {
        let (name_section_offset, name_section_size) = {
            let name_section_h: &SectionHeader = self
                .section_headers
                .get(self.header.s_header_name_entry as usize)
                .ok_or(ElfParseError::SectionNotFound)?;

            if name_section_h.sec_type != SectionType::Strtab {
                return Err(ElfParseError::InvalidNameSecionType);
            }

            (name_section_h.sec_offset, name_section_h.sec_size)
        };

        let section = self.bytes.get_bytes(name_section_offset, name_section_size);

        for sec in &mut self.section_headers {
            if let SectionName::Offset(offset) = sec.name {
                let mut buf = vec![];
                for b in &section[(offset as usize)..] {
                    if b != &b'\0' {
                        buf.push(*b);
                    } else {
                        break;
                    }
                }
                sec.name = SectionName::String(offset, String::from_utf8(buf)?);
            }
        }

        Ok(())
    }
}

pub trait ByteRanges {
    fn get_bytes(&self, offset: u64, size: u64) -> &[u8];

    fn get_bytes_copy<const SIZE: usize>(&self, offset: u64) -> [u8; SIZE];
}

impl ByteRanges for Vec<u8> {
    fn get_bytes(&self, offset: u64, size: u64) -> &[u8] {
        &self[(offset as usize)..((offset + size) as usize)]
    }

    fn get_bytes_copy<const SIZE: usize>(&self, offset: u64) -> [u8; SIZE] {
        let mut buffer: [u8; SIZE] = [0; SIZE];
        buffer.copy_from_slice(self.get_bytes(offset, SIZE as u64));
        buffer
    }
}

impl ByteRanges for &[u8] {
    fn get_bytes(&self, offset: u64, size: u64) -> &[u8] {
        &self[(offset as usize)..((offset + size) as usize)]
    }

    fn get_bytes_copy<const SIZE: usize>(&self, offset: u64) -> [u8; SIZE] {
        let mut buffer: [u8; SIZE] = [0; SIZE];
        buffer.copy_from_slice(self.get_bytes(offset, SIZE as u64));
        buffer
    }
}

impl<const MEM_SIZE: usize> ByteRanges for [u8; MEM_SIZE] {
    fn get_bytes(&self, offset: u64, size: u64) -> &[u8] {
        &self[(offset as usize)..((offset + size) as usize)]
    }

    fn get_bytes_copy<const SIZE: usize>(&self, offset: u64) -> [u8; SIZE] {
        let mut buffer: [u8; SIZE] = [0; SIZE];
        buffer.copy_from_slice(self.get_bytes(offset, SIZE as u64));
        buffer
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Address(pub u64);

impl Debug for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#8x}", self.0)
    }
}

impl<T: Into<u64>> From<T> for Address {
    fn from(value: T) -> Self {
        Self(value.into())
    }
}
