use core::panic;
use std::{
    any::Any,
    collections::HashMap,
    fmt::Debug,
    fs::File,
    io::Write,
    mem,
    ops::{Add, AddAssign, Deref, Range, RangeBounds, Sub},
    rc::Rc,
    sync::{mpsc::Sender, Arc, PoisonError, RwLock, RwLockWriteGuard},
    u8, usize, vec,
};

use elf_load::ByteRanges;
use nohash_hasher::IntMap;

use crate::{
    devices::DeviceInitError,
    hart::{
        privilege::{self, PrivilegeMode},
        Hart,
    },
    vmstate::timer::MTimer,
};

use self::{
    address::{Address, VirtAddress},
    memory_buffer::{MemoryBuffer, MemoryBufferError},
    memory_map::{MemoryMap, MemoryMapError, MemoryRegion},
    paging::{walk_page_table, AccessContext, AddressTranslationMode, PageError, Satp},
    pmp::{AccessMode, PmpCfg, PMP},
};

pub mod address;
pub mod memory_buffer;
mod memory_map;
pub mod paging;
pub mod pmp;
#[cfg(test)]
mod tests;

pub const KB: usize = 1024;
pub const MB: usize = 1024 * KB;

type DeviceRegionId = usize;

pub struct Memory {
    main_buffer: MainMemoryBuffer,
    memory_map: MemoryMap,
    // device_regions: IntMap<usize, Arc<RwLock<DeviceMemory>>>,
    device_regions: IntMap<DeviceRegionId, Arc<RwLock<dyn MemoryBuffer>>>,
    reservations: IntMap<u64, Range<Address>>,
    next_region_id: DeviceRegionId,
}

pub struct MainMemoryBuffer(Box<[u8]>);

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
    UnalignedWrite(Address),
    UnalignedRead(Address),
    OutOfMemory,
    PmpDeniedRead,
    PmpDeniedWrite,
    PmpDeniedFetch,
    PageFaultRead,
    PageFaultWrite,
    PageFaultFetch,
    DeviceMemoryPoison,
    LoadAtomicsUnsupported,
    StoreAtomicsUnsupported,
    FetchUnsupported,
}

// impl Debug for Memory {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         // writeln!(f, "range: {:?}", self.mem_range);
//         for c in self.mem.chunks(32) {
//             for b in c {
//                 write!(f, "{:02X} ", b)?;
//             }
//             writeln!(f)?;
//         }
//         Ok(())
//     }
// }

impl MainMemoryBuffer {
    pub fn new<const SIZE: usize>() -> Self {
        Self(vec![0u8; SIZE].into_boxed_slice())
    }
}

impl MemoryBuffer for MainMemoryBuffer {
    fn size(&self) -> u64 {
        self.0.len() as u64
    }

    fn write_bytes(&mut self, bytes: &[u8], addr: Address) -> Result<(), MemoryBufferError> {
        self.0[addr.into()..(addr + bytes.len() as u64).into()].copy_from_slice(bytes);
        Ok(())
    }

    fn read_bytes(&self, addr: Address, size: usize) -> Result<Vec<u8>, MemoryBufferError> {
        Ok(self.0.deref().get_bytes(addr.into(), size as u64).to_vec())
    }
}

impl Memory {
    pub fn new<const SIZE: usize>() -> Self {
        let mem = vec![0u8; SIZE].into_boxed_slice();
        Self {
            main_buffer: MainMemoryBuffer::new::<SIZE>(),
            memory_map: MemoryMap::new(0x80000000u64.into()..=(0x80000000u64 + SIZE as u64).into()),
            device_regions: IntMap::default(),
            reservations: IntMap::default(),
            next_region_id: 0,
        }
    }

    /// NOTE, does not do atomic checks, pmp checks or page table walks
    pub fn write_bytes(&mut self, bytes: &[u8], addr: Address) -> Result<(), MemoryError> {
        match self.memory_map.fit(addr..(addr + bytes.len() as u64)) {
            Ok(r) => match r {
                MemoryRegion::Ram(r) => {
                    self.main_buffer.write_bytes(bytes, addr - *r.start());
                    Ok(())
                }
                MemoryRegion::Rom(r) => todo!(),
                MemoryRegion::IO(o, r) => self.device_regions[o]
                    .write()?
                    .write_bytes(bytes, addr - *r.start())
                    .map_err(Into::into),
            },
            Err(MemoryMapError::TooLarge) => Err(MemoryError::OutOfMemory),
            Err(MemoryMapError::OutOfBounds) => Err(MemoryError::OutOfBoundsWrite(addr)),
            Err(_) => unreachable!(),
        }
    }

    /// NOTE, does not do atomic checks, pmp checks or page table walks
    pub fn read_bytes(&self, addr: Address, size: usize) -> Result<Vec<u8>, MemoryError> {
        match self.memory_map.fit(addr..(addr + size as u64)) {
            Ok(r) => match r {
                MemoryRegion::Ram(r) => Ok(self.main_buffer.read_bytes(addr - *r.start(), size)?),
                MemoryRegion::Rom(r) => todo!(),
                MemoryRegion::IO(o, r) => self.device_regions[o]
                    .read()?
                    .read_bytes(addr - *r.start(), size)
                    .map_err(Into::into),
            },
            Err(_) => Err(MemoryError::OutOfBoundsRead(addr)),
        }
    }

    /// NOTE, does not do atomic checks, pmp checks or page table walks
    pub fn fetch(&self, addr: Address, privilege: PrivilegeMode) -> Result<u32, MemoryError> {
        match self.memory_map.fit(addr..(addr + 4u64)) {
            Ok(r) => match r {
                MemoryRegion::Ram(r) => {
                    let idx = addr - *r.start();
                    Ok(u32::from_le_bytes(
                        self.main_buffer
                            .read_bytes(addr - *r.start(), 4)?
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
            },
            Err(_) => Err(MemoryError::OutOfBoundsRead(addr)),
        }
    }

    pub fn add_device_memory<M: MemoryBuffer + 'static>(
        &mut self,
        base: Address,
        buf: M,
    ) -> Result<Arc<RwLock<M>>, DeviceInitError> {
        let id = self.next_region_id;
        self.next_region_id += 1;
        match self
            .memory_map
            .add_region(MemoryRegion::IO(id, base..=(base + buf.size())))
        {
            Ok(_) => {
                let mem = Arc::new(RwLock::new(buf));
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

    #[deprecated]
    pub fn register_handle(&mut self, dev_id: usize) -> () {
        unimplemented!()
        // MemoryRegisterHandle::new(self, dev_id)
    }

    #[deprecated]
    fn add_register(&mut self, owner: usize, reg: (), addr: Address) -> Result<(), MemoryMapError> {
        unimplemented!()
        // self.memory_map
        //     .add_region(MemoryRegion::Register(owner, addr))?;
        // self.registers.insert(addr, reg);
        // Ok(())
    }

    // pub fn get_device_memory(
    //     &mut self,
    //     id: &usize,
    // ) -> Result<Option<RwLockWriteGuard<dyn MemoryBuffer + '_>>, MemoryError> {
    //     if let Some(mem) = self.device_regions.get_mut(id) {
    //         Ok(Some(mem.write()?))
    //     } else {
    //         Ok(None)
    //     }
    // }

    pub fn dump(&self) {
        unimplemented!()
        // let mut w = File::create("./mem.dump").unwrap();
        // writeln!(&mut w, "{:?}", self);
    }

    pub fn get_map(&self) -> &MemoryMap {
        &self.memory_map
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
                    // let mut w = File::create("./mem.dump").unwrap();
                    // writeln!(&mut w, "{:?}", &self.mem);
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
        self.mem.write_bytes(bytes, addr)
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
        self.mem.read_bytes(addr, size)
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
                    // let mut w = File::create("./mem.dump").unwrap();
                    // writeln!(&mut w, "{:?}", &self.mem);
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
        self.mem.fetch(addr, self.privilege)
    }

    fn read_phys(&self, addr: Address, size: usize) -> Result<Vec<u8>, MemoryError> {
        self.mem.read_bytes(addr, size)
    }
}

impl<T> From<PoisonError<T>> for MemoryError {
    fn from(value: PoisonError<T>) -> Self {
        Self::DeviceMemoryPoison
    }
}
