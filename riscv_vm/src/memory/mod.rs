use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Range, Sub},
    vec,
};

use elf_load::ByteRanges;

use registers::IntRegister;

use self::address::Address;

pub mod address;
pub mod registers;
#[cfg(test)]
mod tests;

pub const KB: usize = 1024;
pub const MB: usize = 1024 * KB;

#[derive(Debug)]
pub struct Memory<const SIZE: usize> {
    mem: [u8; SIZE],
    mem_range: Range<Address>,
}

#[derive(Debug)]
pub enum MemoryError {
    OutOfBoundsWrite,
    OutOfBoundsRead,
    OutOfMemory,
}

impl<const SIZE: usize> Memory<SIZE> {
    pub fn new() -> Self {
        Self {
            mem: [0; SIZE],
            mem_range: 0x80000000u64.into()..(0x80000000u64 + SIZE as u64).into(),
        }
    }

    pub fn write_bytes(&mut self, bytes: &[u8], addr: Address) -> Result<(), MemoryError> {
        if (self.mem_range.contains(&addr)) {
            let idx = addr - self.mem_range.start;
            if <Address as Into<usize>>::into(idx) > self.mem.len() {
                return Err(MemoryError::OutOfBoundsWrite);
            }
            if <Address as Into<usize>>::into(idx) + bytes.len() > self.mem.len() {
                return Err(MemoryError::OutOfMemory);
            }
            self.mem[idx.into()..(<Address as Into<usize>>::into(idx) + bytes.len())]
                .copy_from_slice(bytes);
        }
        Ok(())
    }

    pub fn read_bytes(&self, addr: Address, size: usize) -> Result<&[u8], MemoryError> {
        if (self.mem_range.contains(&addr)) {
            let idx = addr - self.mem_range.start;
            if <Address as Into<usize>>::into(idx) + size < self.mem.len() {
                Ok(&self.mem.get_bytes(idx.into(), size as u64))
            } else {
                Err(MemoryError::OutOfBoundsRead)
            }
        } else {
            Err(MemoryError::OutOfBoundsRead)
        }
    }

    pub fn write_reg(&mut self) {}
}
