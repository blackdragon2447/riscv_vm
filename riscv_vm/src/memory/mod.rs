use core::panic;
use std::{
    collections::HashMap,
    fmt::Debug,
    fs::File,
    io::Write,
    mem,
    ops::{Add, AddAssign, Deref, Range, RangeBounds, Sub},
    sync::{mpsc::Sender, Arc, PoisonError, RwLock, RwLockWriteGuard},
    u8, usize, vec,
};

use elf_load::ByteRanges;
use nohash_hasher::IntMap;

use crate::{
    devices::{
        event_bus::{DeviceEvent, DeviceEventType},
        DeviceInitError,
    },
    hart::{
        privilege::{self, PrivilegeMode},
        Hart,
    },
};

use self::{
    address::{Address, VirtAddress},
    memory_map::{MemoryMap, MemoryMapError, MemoryRegion},
    paging::{walk_page_table, AccessContext, AddressTranslationMode, PageError, Satp},
    pmp::{AccessMode, PmpCfg, PMP},
    registers::{MemoryRegisterHandle, Register},
};

pub mod address;
mod memory_map;
pub mod paging;
pub mod pmp;
pub mod registers;
#[cfg(test)]
mod tests;

pub const KB: usize = 1024;
pub const MB: usize = 1024 * KB;

pub struct DeviceMemory(Range<Address>, Vec<u8>);

pub struct Memory {
    mem: Box<[u8]>,
    memory_map: MemoryMap,
    registers: IntMap<Address, Register>,
    device_regions: IntMap<usize, Arc<RwLock<DeviceMemory>>>,
    reservations: IntMap<u64, Range<Address>>,
    device_event_bus: Sender<DeviceEvent>,
}

pub struct MemoryWindow<'a> {
    mem: &'a mut Memory,
    hartid: u64,
    privilege: PrivilegeMode,
    pmp: Option<&'a PMP>,
    paging: Satp,
    mxr: bool,
    sum: bool,
}

#[derive(Debug, PartialEq, Eq)]
pub enum MemoryError {
    OutOfBoundsWrite(Address),
    OutOfBoundsRead(Address),
    OutOfMemory,
    PmpDeniedRead,
    PmpDeniedWrite,
    PmpDeniedFetch,
    PageFaultRead,
    PageFaultWrite,
    PageFaultFetch,
    DeviceMemoryPoison,
}

impl Debug for Memory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // writeln!(f, "range: {:?}", self.mem_range);
        for c in self.mem.chunks(32) {
            for b in c {
                write!(f, "{:02X} ", b)?;
            }
            writeln!(f);
        }
        Ok(())
    }
}

impl Memory {
    pub fn new<const SIZE: usize>(device_event_bus: Sender<DeviceEvent>) -> Self {
        let mem = vec![0u8; SIZE].into_boxed_slice();
        Self {
            mem,
            memory_map: MemoryMap::new(
                (0x80000000u64.into()..(0x80000000u64 + SIZE as u64).into()),
            ),
            registers: IntMap::default(),
            device_regions: IntMap::default(),
            reservations: IntMap::default(),
            device_event_bus,
        }
    }

    /// NOTE, does not do atomic checks, pmp checks or page table walks
    pub fn write_bytes(
        &mut self,
        bytes: &[u8],
        addr: Address,
        privilege: PrivilegeMode,
        pmp: Option<&PMP>,
    ) -> Result<(), MemoryError> {
        match self.memory_map.fit(addr..(addr + bytes.len() as u64)) {
            Ok(r) => match r {
                MemoryRegion::Ram(r) => {
                    let idx = addr - r.start;
                    self.mem[idx.into()..(<Address as Into<usize>>::into(idx) + bytes.len())]
                        .copy_from_slice(bytes);
                    Ok(())
                }
                MemoryRegion::Rom(r) => todo!(),
                MemoryRegion::IO(o, r) => self.device_regions[o].write()?.write_bytes(bytes, addr),
                MemoryRegion::Register(o, a) => {
                    let value = match bytes.len() {
                        0 => [0; 8],
                        i @ 1..=8 => {
                            let mut value = [0u8; 8];
                            value[0..i].copy_from_slice(bytes);
                            value
                        }
                        i => {
                            let mut value = [0u8; 8];
                            value[0..8].copy_from_slice(&bytes[0..8]);
                            value
                        }
                    };
                    self.registers
                        .get_mut(a)
                        .unwrap()
                        .set(u64::from_le_bytes(value));
                    self.device_event_bus
                        .send(DeviceEvent(*o, DeviceEventType::RegisterWrite(*a)));
                    Ok(())
                }
            },
            Err(MemoryMapError::TooLarge) => Err(MemoryError::OutOfMemory),
            Err(MemoryMapError::OutOfBounds) => Err(MemoryError::OutOfBoundsWrite(addr)),
            Err(_) => unreachable!(),
        }
    }

    /// NOTE, does not do atomic checks, pmp checks or page table walks
    pub fn read_bytes(
        &self,
        addr: Address,
        size: usize,
        privilege: PrivilegeMode,
        pmp: Option<&PMP>,
    ) -> Result<Vec<u8>, MemoryError> {
        match self.memory_map.fit(addr..(addr + size as u64)) {
            Ok(r) => match r {
                MemoryRegion::Ram(r) => {
                    let idx = addr - r.start;
                    Ok(self.mem.deref().get_bytes(idx.into(), size as u64).to_vec())
                }
                MemoryRegion::Rom(r) => todo!(),
                MemoryRegion::IO(o, r) => self.device_regions[o].read()?.read_bytes(addr, size),
                MemoryRegion::Register(o, a) => todo!(),
            },
            Err(_) => Err(MemoryError::OutOfBoundsRead(addr)),
        }
    }

    /// NOTE, does not do atomic checks, pmp checks or page table walks
    pub fn fetch(
        &self,
        addr: Address,
        privilege: PrivilegeMode,
        pmp: Option<&PMP>,
    ) -> Result<u32, MemoryError> {
        match self.memory_map.fit(addr..(addr + 4u64)) {
            Ok(r) => match r {
                MemoryRegion::Ram(r) => {
                    let idx = addr - r.start;
                    Ok(u32::from_le_bytes(
                        self.mem
                            .deref()
                            .get_bytes(idx.into(), 4)
                            .try_into()
                            .unwrap(),
                    ))
                }
                MemoryRegion::Rom(r) => todo!(),
                MemoryRegion::IO(o, r) => Ok(u32::from_le_bytes(
                    self.device_regions[o]
                        .read()?
                        .read_bytes(addr, 4)
                        .map_err(|_| MemoryError::OutOfBoundsRead(addr))?
                        .try_into()
                        .unwrap(),
                )),
                MemoryRegion::Register(o, a) => todo!(),
            },
            Err(_) => Err(MemoryError::OutOfBoundsRead(addr)),
        }
    }

    pub fn add_device_memory(
        &mut self,
        id: usize,
        mem: DeviceMemory,
    ) -> Result<Arc<RwLock<DeviceMemory>>, DeviceInitError> {
        match self
            .memory_map
            .add_region(MemoryRegion::IO(id, mem.0.clone()))
        {
            Ok(_) => {
                let mem = Arc::new(RwLock::new(mem));
                self.device_regions.insert(id, mem.clone());
                Ok(mem)
            }
            Err(MemoryMapError::RegionOverlap) => Err(DeviceInitError::MemoryOverlap),
            Err(_) => unreachable!(),
        }
    }

    pub fn window<'a>(&'a mut self, hart: &'a Hart) -> MemoryWindow {
        let (mxr, sum) = hart.get_csr().get_mxr_sum();
        MemoryWindow {
            mem: self,
            hartid: hart.get_hart_id(),
            privilege: if hart.get_csr().get_status().mprv {
                hart.get_csr().get_status().mpp
            } else {
                hart.privilege()
            },
            pmp: hart.pmp_enable().then(|| &hart.get_csr().pmp),
            paging: hart.get_csr().get_satp(),
            mxr,
            sum,
        }
    }

    pub fn register_handle<'a>(&'a mut self, dev_id: usize) -> MemoryRegisterHandle {
        MemoryRegisterHandle::new(self, dev_id)
    }

    fn add_register(
        &mut self,
        owner: usize,
        reg: Register,
        addr: Address,
    ) -> Result<(), MemoryMapError> {
        self.memory_map
            .add_region(MemoryRegion::Register(owner, addr))?;
        self.registers.insert(addr, reg);
        Ok(())
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

    pub fn dump(&self) {
        let mut w = File::create("./mem.dump").unwrap();
        writeln!(&mut w, "{:?}", self);
    }
}

impl MemoryRegisterHandle<'_> {
    pub fn add_register(&mut self, addr: Address, permission: PrivilegeMode) -> bool {
        self.memory_ref
            .add_register(self.dev_id, Register::new(permission), addr)
            .is_ok()
    }
}

impl MemoryWindow<'_> {
    pub fn write_bytes(&mut self, bytes: &[u8], addr: Address) -> Result<(), MemoryError> {
        let addr = if self.paging.mode != AddressTranslationMode::Bare
            && self.privilege != PrivilegeMode::Machine
        {
            match walk_page_table(
                VirtAddress::from_address(addr, self.paging.mode),
                self.paging,
                self,
                AccessContext {
                    mode: AccessMode::Write,
                    privilege: self.privilege,
                    mxr: self.mxr,
                    sum: self.sum,
                },
            )
            .map_err(|e| match e {
                PageError::AccessFault => MemoryError::PmpDeniedWrite,
                PageError::PageFault => MemoryError::PageFaultWrite,
            }) {
                Ok(a) => a,
                Err(e) => {
                    let mut w = File::create("./mem.dump").unwrap();
                    writeln!(&mut w, "{:?}", &self.mem);
                    return Err(e);
                }
            }
        } else {
            addr
        };
        if !self.pmp.map_or_else(
            || true,
            |pmp| pmp.check(addr, self.privilege, AccessMode::Write),
        ) {
            return Err(MemoryError::PmpDeniedWrite);
        };
        // Remove all reservations that
        // contain the address we write to
        self.mem.reservations.retain(|_, v| {
            let range = addr..(addr + bytes.len() as u64);
            v.start >= range.end || range.start >= v.end
        });
        self.mem.write_bytes(bytes, addr, self.privilege, self.pmp)
    }

    pub fn read_bytes(&mut self, addr: Address, size: usize) -> Result<Vec<u8>, MemoryError> {
        let addr = if self.paging.mode != AddressTranslationMode::Bare
            && self.privilege != PrivilegeMode::Machine
        {
            walk_page_table(
                VirtAddress::from_address(addr, self.paging.mode),
                self.paging,
                self,
                AccessContext {
                    mode: AccessMode::Write,
                    privilege: self.privilege,
                    mxr: self.mxr,
                    sum: self.sum,
                },
            )
            .map_err(|e| match e {
                PageError::AccessFault => MemoryError::PmpDeniedRead,
                PageError::PageFault => MemoryError::PageFaultRead,
            })?
        } else {
            addr
        };
        if !self.pmp.map_or_else(
            || true,
            |pmp| pmp.check(addr, self.privilege, AccessMode::Read),
        ) {
            return Err(MemoryError::PmpDeniedRead);
        }
        // Remove all reservations that
        // contain the address we write to
        self.mem.reservations.retain(|_, v| {
            let range = addr..(addr + size as u64);
            v.start >= range.end || range.start >= v.end
        });
        self.mem.read_bytes(addr, size, self.privilege, self.pmp)
    }

    pub fn write_conditional(&mut self, bytes: &[u8], addr: Address) -> Result<bool, MemoryError> {
        // A reservation exists for this hart and is for the address and size we want to write to
        dbg!(&self.mem.reservations);
        dbg!(addr..(addr + bytes.len() as u64));
        if let Some(r) = self.mem.reservations.get(&self.hartid) {
            if *r == (addr..(addr + bytes.len() as u64)) {
                self.write_bytes(bytes, addr).map(|_| true)
            } else {
                Ok(false)
            }
        } else {
            Ok(false)
        }
    }

    pub fn read_reserve(&mut self, addr: Address, size: usize) -> Result<Vec<u8>, MemoryError> {
        let bytes = self.read_bytes(addr, size)?;
        self.mem
            .reservations
            .insert(self.hartid, addr..(addr + size as u64));
        Ok(bytes)
    }

    #[inline]
    pub fn atomic_operation_w(
        &mut self,
        addr: Address,
        rs: i32,
        op: fn(orig: i32, rs: i32) -> i32,
    ) -> Result<i32, MemoryError> {
        let bytes = self.read_bytes(addr, 4)?;
        let orig = i32::from_le_bytes(bytes.try_into().unwrap());
        self.write_bytes(&op(rs, orig).to_le_bytes(), addr);
        Ok(orig)
    }

    #[inline]
    pub fn atomic_operation_d(
        &mut self,
        addr: Address,
        rs: i64,
        op: fn(orig: i64, rs: i64) -> i64,
    ) -> Result<i64, MemoryError> {
        let bytes = self.read_bytes(addr, 8)?;
        let orig = i64::from_le_bytes(bytes.try_into().unwrap());
        self.write_bytes(&op(rs, orig).to_le_bytes(), addr);
        Ok(orig)
    }

    pub fn fetch(&mut self, addr: Address) -> Result<u32, MemoryError> {
        let addr = if self.paging.mode != AddressTranslationMode::Bare
            && self.privilege != PrivilegeMode::Machine
        {
            match walk_page_table(
                VirtAddress::from_address(addr, self.paging.mode),
                self.paging,
                self,
                AccessContext {
                    mode: AccessMode::Exec,
                    privilege: self.privilege,
                    mxr: self.mxr,
                    sum: self.sum,
                },
            )
            .map_err(|e| match e {
                PageError::AccessFault => MemoryError::PmpDeniedFetch,
                PageError::PageFault => MemoryError::PageFaultFetch,
            }) {
                Ok(a) => a,
                Err(e) => {
                    let mut w = File::create("./mem.dump").unwrap();
                    writeln!(&mut w, "{:?}", &self.mem);
                    return Err(e);
                }
            }
        } else {
            addr
        };
        if !self.pmp.map_or_else(
            || true,
            |pmp| pmp.check(addr, self.privilege, AccessMode::Read),
        ) {
            return Err(MemoryError::PmpDeniedRead);
        }
        // Remove all reservations that
        // contain the address we write to
        self.mem.reservations.retain(|_, v| {
            let range = addr..(addr + 4);
            v.start >= range.end || range.start >= v.end
        });
        self.mem.fetch(addr, self.privilege, self.pmp)
    }

    pub(self) fn read_phys(&self, addr: Address, size: usize) -> Result<Vec<u8>, MemoryError> {
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
                return Err(MemoryError::OutOfBoundsWrite(addr));
            }
            if <Address as Into<usize>>::into(idx) + bytes.len() > self.1.len() {
                return Err(MemoryError::OutOfMemory);
            }
            self.1[idx.into()..(<Address as Into<usize>>::into(idx) + bytes.len())]
                .copy_from_slice(bytes);
        } else {
            return Err(MemoryError::OutOfBoundsWrite(addr));
        }
        Ok(())
    }

    pub fn read_bytes(&self, addr: Address, size: usize) -> Result<Vec<u8>, MemoryError> {
        if (self.0.contains(&addr)) {
            let idx = addr - self.0.start;
            if <Address as Into<usize>>::into(idx) + size < self.1.len() {
                Ok(self.1.get_bytes(idx.into(), size as u64).to_vec())
            } else {
                Err(MemoryError::OutOfBoundsRead(addr))
            }
        } else {
            Err(MemoryError::OutOfBoundsRead(addr))
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
