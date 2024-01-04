use std::{
    fmt::Debug,
    ops::{Add, Sub},
    vec,
};

use elf_load::BitRanges;

#[derive(Clone, Copy)]
pub struct Address(u64);

const KB: usize = 1024;
const MB: usize = 1024 ^ 2;

pub struct Memory {
    mem: Vec<u8>,
    mem_start: Address,
}

impl Memory {
    pub fn new_kb(size: usize) -> Self {
        let mut mem = Vec::new();
        mem.resize(size * KB, 0);
        Self {
            mem,
            mem_start: 0x80000000.into(),
        }
    }

    pub fn new_mb(size: usize) -> Self {
        let mut mem = Vec::new();
        mem.resize(size * MB, 0);
        Self {
            mem,
            mem_start: 0x80000000.into(),
        }
    }

    pub fn write_bytes(&mut self, bytes: &[u8], addr: Address) {
        let idx = addr - self.mem_start;
        if <Address as Into<usize>>::into(idx) + bytes.len() < self.mem.len() {
            self.mem[idx.into()..(<Address as Into<usize>>::into(idx) + bytes.len())]
                .copy_from_slice(bytes);
        }
    }

    pub fn read_bytes(&self, addr: Address, size: usize) -> Option<&[u8]> {
        let idx = addr - self.mem_start;
        if <Address as Into<usize>>::into(idx) + size < self.mem.len() {
            Some(&self.mem.get_bytes(idx.into(), size as u64))
        } else {
            None
        }
    }
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
