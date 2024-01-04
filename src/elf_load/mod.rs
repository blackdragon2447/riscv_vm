use self::{program_header::ProgramHeader, section_header::SectionHeader};

mod data;
pub mod elf_header;
mod program_header;
mod section_header;

struct Elf {
    header: elf_header::ElfHeader,
    program_headers: Vec<ProgramHeader>,
    section_headers: Vec<SectionHeader>,
    bytes: Vec<u8>,
}

#[derive(Debug)]
pub enum ElfParseError {
    InvalidMagic,
    InvalidBitness(u8),
    InvalidEndianess,
    InvalidVersion(u8),
    InvalidAbi(u8),
    InvalidObjType(u16),
    InvalidASI,
    ReservedASI,
    InvalidVersionOrig(u32),
    InvalidSize(u16),
}

impl Elf {
    fn from_bytes(bytes: Vec<u8>) -> Result<Elf, ElfParseError> {
        unimplemented!()
    }
}

pub trait BitRanges {
    fn get_bytes(&self, offset: u64, size: u64) -> &[u8];

    fn get_bytes_copy<const SIZE: usize>(&self, offset: u64) -> [u8; SIZE];
}

impl BitRanges for Vec<u8> {
    fn get_bytes(&self, offset: u64, size: u64) -> &[u8] {
        let slice = &self[(offset as usize)..((offset + size) as usize)];
        slice
    }

    fn get_bytes_copy<const SIZE: usize>(&self, offset: u64) -> [u8; SIZE] {
        let mut buffer: [u8; SIZE] = [0; SIZE];
        buffer.copy_from_slice(self.get_bytes(offset, SIZE as u64));
        return buffer;
    }
}
