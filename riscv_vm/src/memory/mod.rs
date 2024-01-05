use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Sub},
    vec,
};

use elf_load::ByteRanges;

use registers::IntRegister;

pub mod registers;
#[cfg(test)]
mod tests;

#[derive(Clone, Copy)]
pub struct Address(u64);

pub const KB: usize = 1024;
pub const MB: usize = 1024 * KB;

#[derive(Debug)]
pub struct Memory<const SIZE: usize> {
    mem: [u8; SIZE],
    mem_start: Address,
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
            mem_start: 0x80000000.into(),
        }
    }

    pub fn write_bytes(&mut self, bytes: &[u8], addr: Address) -> Result<(), MemoryError> {
        let idx = addr - self.mem_start;
        if <Address as Into<usize>>::into(idx) > self.mem.len() {
            return Err(MemoryError::OutOfBoundsWrite);
        }
        if <Address as Into<usize>>::into(idx) + bytes.len() > self.mem.len() {
            return Err(MemoryError::OutOfMemory);
        }
        self.mem[idx.into()..(<Address as Into<usize>>::into(idx) + bytes.len())]
            .copy_from_slice(bytes);
        Ok(())
    }

    pub fn read_bytes(&self, addr: Address, size: usize) -> Result<&[u8], MemoryError> {
        let idx = addr - self.mem_start;
        if <Address as Into<usize>>::into(idx) + size < self.mem.len() {
            Ok(&self.mem.get_bytes(idx.into(), size as u64))
        } else {
            Err(MemoryError::OutOfBoundsRead)
        }
    }

    pub fn write_reg(&mut self) {}
}

impl Debug for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#8x}", self.0)
    }
}

impl Add for Address {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign for Address {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0
    }
}

impl Sub for Address {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl Into<usize> for Address {
    fn into(self) -> usize {
        self.0 as usize
    }
}

impl Into<u64> for Address {
    fn into(self) -> u64 {
        self.0
    }
}

impl From<u64> for Address {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl From<elf_load::Address> for Address {
    fn from(value: elf_load::Address) -> Self {
        Self(value.0)
    }
}
