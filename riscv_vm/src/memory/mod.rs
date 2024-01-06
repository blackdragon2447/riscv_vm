use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Range, Sub},
    vec,
};

use elf_load::ByteRanges;

use registers::IntRegister;

use self::{
    address::Address,
    mem_map_device::{MemMapDevice, MemMapDeviceState},
};

pub mod address;
pub mod mem_map_device;
pub mod registers;
#[cfg(test)]
mod tests;

pub const KB: usize = 1024;
pub const MB: usize = 1024 * KB;

pub struct Memory<const SIZE: usize> {
    mem: [u8; SIZE],
    mem_range: Range<Address>,
    devices: Vec<MemMapDeviceState>,
}

#[derive(Debug)]
pub enum MemoryError {
    OutOfBoundsWrite,
    OutOfBoundsRead(Address),
    OutOfMemory,
}

impl<const SIZE: usize> Default for Memory<SIZE> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const SIZE: usize> Memory<SIZE> {
    pub fn new() -> Self {
        Self {
            mem: [0; SIZE],
            mem_range: 0x80000000u64.into()..(0x80000000u64 + SIZE as u64).into(),
            devices: Vec::new(),
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
        } else {
            for dev in &mut self.devices {
                if dev.in_range(addr) {
                    return dev.write_bytes(bytes, addr);
                }
            }
        }
        Ok(())
    }

    pub fn read_bytes(&self, addr: Address, size: usize) -> Result<&[u8], MemoryError> {
        if (self.mem_range.contains(&addr)) {
            let idx = addr - self.mem_range.start;
            if <Address as Into<usize>>::into(idx) + size < self.mem.len() {
                Ok(self.mem.get_bytes(idx.into(), size as u64))
            } else {
                Err(MemoryError::OutOfBoundsRead(addr))
            }
        } else {
            for dev in &self.devices {
                if dev.in_range(addr) {
                    return dev.read_bytes(addr, size);
                }
            }
            Err(MemoryError::OutOfBoundsRead(addr))
        }
    }
}
