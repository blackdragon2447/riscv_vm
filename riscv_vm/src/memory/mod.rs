use std::{
    collections::HashMap,
    fmt::Debug,
    ops::{Add, AddAssign, Deref, Range, RangeBounds, Sub},
    sync::{Arc, PoisonError, RwLock, RwLockWriteGuard},
    vec,
};

use elf_load::ByteRanges;

use registers::IntRegister;

use crate::{
    devices::DeviceInitError,
    hart::{
        privilege::{self, PrivilegeMode},
        Hart,
    },
};

use self::{
    address::Address,
    pmp::{AccessMode, PmpCfg, PMP},
};

pub mod address;
pub mod pmp;
pub mod registers;
#[cfg(test)]
mod tests;

pub const KB: usize = 1024;
pub const MB: usize = 1024 * KB;

pub struct DeviceMemory(Range<Address>, Vec<u8>);

pub struct Memory {
    mem: Box<[u8]>,
    mem_range: Range<Address>,
    device_regions: HashMap<usize, Arc<RwLock<DeviceMemory>>>,
}

pub struct MemoryWindow<'a> {
    mem: &'a mut Memory,
    privilege: PrivilegeMode,
    pmp: Option<&'a PMP>,
}

#[derive(Debug)]
pub enum MemoryError {
    OutOfBoundsWrite(Address, Range<Address>),
    OutOfBoundsRead(Address, Range<Address>),
    OutOfMemory,
    FetchError,
    PmpDeniedRead,
    PmpDeniedWrite,
    DeviceMemoryPoison,
}

impl Default for Memory {
    fn default() -> Self {
        Self::new::<{ 4 * KB }>()
    }
}

impl Memory {
    pub fn new<const SIZE: usize>() -> Self {
        let mem = vec![0u8; SIZE].into_boxed_slice();
        Self {
            mem,
            mem_range: 0x80000000u64.into()..(0x80000000u64 + SIZE as u64).into(),
            device_regions: HashMap::new(),
        }
    }

    pub fn write_bytes(
        &mut self,
        bytes: &[u8],
        addr: Address,
        privilege: PrivilegeMode,
        pmp: Option<&PMP>,
    ) -> Result<(), MemoryError> {
        if !pmp.map_or_else(|| true, |pmp| pmp.check(addr, privilege, AccessMode::Write)) {
            return Err(MemoryError::PmpDeniedWrite);
        }
        if (self.mem_range.contains(&addr)) {
            let idx = addr - self.mem_range.start;
            if <Address as Into<usize>>::into(idx) > self.mem.len() {
                return Err(MemoryError::OutOfBoundsWrite(addr, self.mem_range.clone()));
            }
            if <Address as Into<usize>>::into(idx) + bytes.len() > self.mem.len() {
                return Err(MemoryError::OutOfMemory);
            }
            self.mem[idx.into()..(<Address as Into<usize>>::into(idx) + bytes.len())]
                .copy_from_slice(bytes);
            Ok(())
        } else {
            for dev in &mut self.device_regions.values_mut() {
                let mut dev = dev.write()?;
                if dev.0.contains(&addr) {
                    return dev.write_bytes(bytes, addr);
                }
            }
            Err(MemoryError::OutOfBoundsWrite(addr, self.mem_range.clone()))
        }
    }

    pub fn read_bytes(
        &self,
        addr: Address,
        size: usize,
        privilege: PrivilegeMode,
        pmp: Option<&PMP>,
    ) -> Result<Vec<u8>, MemoryError> {
        if !pmp.map_or_else(|| true, |pmp| pmp.check(addr, privilege, AccessMode::Read)) {
            return Err(MemoryError::PmpDeniedRead);
        }
        if (self.mem_range.contains(&addr)) {
            let idx = addr - self.mem_range.start;
            if <Address as Into<usize>>::into(idx) + size < self.mem.len() {
                Ok(self.mem.deref().get_bytes(idx.into(), size as u64).to_vec())
            } else {
                Err(MemoryError::OutOfBoundsRead(addr, self.mem_range.clone()))
            }
        } else {
            for dev in self.device_regions.values() {
                let dev = dev.read()?;
                if dev.0.contains(&addr) {
                    return dev.read_bytes(addr, size);
                }
            }
            Err(MemoryError::OutOfBoundsRead(addr, self.mem_range.clone()))
        }
    }

    pub fn fetch(
        &self,
        addr: Address,
        privilege: PrivilegeMode,
        pmp: Option<&PMP>,
    ) -> Result<u32, MemoryError> {
        if !pmp.map_or_else(|| true, |pmp| pmp.check(addr, privilege, AccessMode::Exec)) {
            return Err(MemoryError::FetchError);
        }
        if (self.mem_range.contains(&addr)) {
            let idx = addr - self.mem_range.start;
            if <Address as Into<usize>>::into(idx) + 4 < self.mem.len() {
                Ok(u32::from_le_bytes(
                    self.mem
                        .deref()
                        .get_bytes(idx.into(), 4)
                        .try_into()
                        .unwrap(),
                ))
            } else {
                Err(MemoryError::FetchError)
            }
        } else {
            for dev in self.device_regions.values() {
                let dev = dev.read()?;
                if dev.0.contains(&addr) {
                    return Ok(u32::from_le_bytes(
                        dev.read_bytes(addr, 4)
                            .map_err(|_| MemoryError::FetchError)?
                            .try_into()
                            .unwrap(),
                    ));
                }
            }
            Err(MemoryError::FetchError)
        }
    }

    pub fn add_device_memory(
        &mut self,
        id: usize,
        mem: DeviceMemory,
    ) -> Result<Arc<RwLock<DeviceMemory>>, DeviceInitError> {
        for dev in &self.device_regions {
            let dev = dev.1.read()?;
            if dev.0.contains(&mem.0.start)
                || dev.0.contains(&mem.0.end)
                || mem.0.contains(&dev.0.start)
                || mem.0.contains(&dev.0.end)
            {
                return Err(DeviceInitError::MemoryOverlap);
            }
        }
        let mem = Arc::new(RwLock::new(mem));
        self.device_regions.insert(id, mem.clone());
        Ok(mem)
    }

    pub fn window<'a>(&'a mut self, hart: &'a Hart) -> MemoryWindow {
        MemoryWindow {
            mem: self,
            privilege: if hart.get_csr().get_status().mprv {
                hart.get_csr().get_status().mpp
            } else {
                hart.privilege()
            },
            pmp: hart.pmp_enable().then(|| &hart.get_csr().pmp),
        }
    }

    pub fn get_device_memory(
        &mut self,
        id: &usize,
    ) -> Result<Option<RwLockWriteGuard<DeviceMemory>>, MemoryError> {
        if let Some(mem) = self.device_regions.get_mut(id) {
            Ok(Some(mem.write()?))
        } else {
            Ok(None)
        }
    }
}

impl MemoryWindow<'_> {
    pub fn write_bytes(&mut self, bytes: &[u8], addr: Address) -> Result<(), MemoryError> {
        self.mem.write_bytes(bytes, addr, self.privilege, self.pmp)
    }

    pub fn read_bytes(&mut self, addr: Address, size: usize) -> Result<Vec<u8>, MemoryError> {
        self.mem.read_bytes(addr, size, self.privilege, self.pmp)
    }
}

impl DeviceMemory {
    pub fn size(&self) -> u64 {
        (self.0.end - self.0.start).into()
    }

    pub fn get_mem(&self) -> &[u8] {
        &self.1
    }

    pub fn get_mem_mut(&mut self) -> &mut [u8] {
        &mut self.1
    }

    pub fn new(size: u64, addr: Address) -> Self {
        Self(addr..(addr + size), vec![0; size as usize])
    }

    pub fn write_bytes(&mut self, bytes: &[u8], addr: Address) -> Result<(), MemoryError> {
        if (self.0.contains(&addr)) {
            let idx = addr - self.0.start;
            if <Address as Into<usize>>::into(idx) > self.1.len() {
                return Err(MemoryError::OutOfBoundsWrite(addr, self.0.clone()));
            }
            if <Address as Into<usize>>::into(idx) + bytes.len() > self.1.len() {
                return Err(MemoryError::OutOfMemory);
            }
            self.1[idx.into()..(<Address as Into<usize>>::into(idx) + bytes.len())]
                .copy_from_slice(bytes);
        } else {
            return Err(MemoryError::OutOfBoundsWrite(addr, self.0.clone()));
        }
        Ok(())
    }

    pub fn read_bytes(&self, addr: Address, size: usize) -> Result<Vec<u8>, MemoryError> {
        if (self.0.contains(&addr)) {
            let idx = addr - self.0.start;
            if <Address as Into<usize>>::into(idx) + size < self.1.len() {
                Ok(self.1.get_bytes(idx.into(), size as u64).to_vec())
            } else {
                Err(MemoryError::OutOfBoundsRead(addr, self.0.clone()))
            }
        } else {
            Err(MemoryError::OutOfBoundsRead(addr, self.0.clone()))
        }
    }

    pub fn start(&self) -> Address {
        self.0.start
    }
}

impl<T> From<PoisonError<T>> for MemoryError {
    fn from(value: PoisonError<T>) -> Self {
        Self::DeviceMemoryPoison
    }
}
