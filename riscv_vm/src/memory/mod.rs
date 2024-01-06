use std::{
    collections::HashMap,
    fmt::Debug,
    ops::{Add, AddAssign, Range, Sub},
    vec,
};

use elf_load::ByteRanges;

use registers::IntRegister;

use crate::devices::DeviceInitError;

use self::address::Address;

pub mod address;
pub mod registers;
#[cfg(test)]
mod tests;

pub const KB: usize = 1024;
pub const MB: usize = 1024 * KB;

pub struct DeviceMemory(Range<Address>, Vec<u8>);

pub struct Memory<const SIZE: usize> {
    mem: [u8; SIZE],
    mem_range: Range<Address>,
    device_regions: HashMap<usize, DeviceMemory>,
}

#[derive(Debug)]
pub enum MemoryError {
    OutOfBoundsWrite,
    OutOfBoundsRead(Address),
    OutOfMemory,
}

impl<'a, const SIZE: usize> Default for Memory<SIZE> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, const SIZE: usize> Memory<SIZE> {
    pub fn new() -> Self {
        Self {
            mem: [0; SIZE],
            mem_range: 0x80000000u64.into()..(0x80000000u64 + SIZE as u64).into(),
            device_regions: HashMap::new(),
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
            for dev in &mut self.device_regions.values_mut() {
                if dev.0.contains(&addr) {
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
            for dev in self.device_regions.values() {
                if dev.0.contains(&addr) {
                    return dev.read_bytes(addr, size);
                }
            }
            Err(MemoryError::OutOfBoundsRead(addr))
        }
    }

    pub fn add_device_memory(
        &mut self,
        id: usize,
        mem: DeviceMemory,
    ) -> Result<(), DeviceInitError> {
        for dev in &self.device_regions {
            if dev.1 .0.contains(&mem.0.start)
                || dev.1 .0.contains(&mem.0.end)
                || mem.0.contains(&dev.1 .0.start)
                || mem.0.contains(&dev.1 .0.end)
            {
                return Err(DeviceInitError::MemoryOverlap);
            }
        }
        self.device_regions.insert(id, mem);
        Ok(())
    }

    pub fn get_device_memory(&mut self, id: &usize) -> Option<&mut DeviceMemory> {
        self.device_regions.get_mut(id)
    }
}

impl DeviceMemory {
    pub fn size(&self) -> u64 {
        (self.0.end - self.0.start).into()
    }

    pub fn get_mem(&self) -> &[u8] {
        &self.1[..]
    }

    pub fn get_mem_mut(&mut self) -> &mut [u8] {
        &mut self.1[..]
    }

    pub fn new(size: u64, addr: Address) -> Self {
        let mut mem = Vec::with_capacity(size as usize);
        for _ in 0..size {
            mem.push(0);
        }
        Self(addr..(addr + size), mem)
    }

    pub fn write_bytes(&mut self, bytes: &[u8], addr: Address) -> Result<(), MemoryError> {
        if (self.0.contains(&addr)) {
            let idx = addr - self.0.start;
            if <Address as Into<usize>>::into(idx) > self.1.len() {
                return Err(MemoryError::OutOfBoundsWrite);
            }
            if <Address as Into<usize>>::into(idx) + bytes.len() > self.1.len() {
                return Err(MemoryError::OutOfMemory);
            }
            self.1[idx.into()..(<Address as Into<usize>>::into(idx) + bytes.len())]
                .copy_from_slice(bytes);
        } else {
            return Err(MemoryError::OutOfBoundsWrite);
        }
        Ok(())
    }

    pub fn read_bytes(&self, addr: Address, size: usize) -> Result<&[u8], MemoryError> {
        if (self.0.contains(&addr)) {
            let idx = addr - self.0.start;
            if <Address as Into<usize>>::into(idx) + size < self.1.len() {
                Ok(self.1.get_bytes(idx.into(), size as u64))
            } else {
                Err(MemoryError::OutOfBoundsRead(addr))
            }
        } else {
            Err(MemoryError::OutOfBoundsRead(addr))
        }
    }
}
