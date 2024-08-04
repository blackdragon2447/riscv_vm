use std::{rc::Rc, sync::Mutex};

use enumflags2::BitFlags;
use nohash_hasher::IntMap;

use crate::{
    hart::{privilege::PrivilegeMode, Hart},
    memory::memory_buffer::{MemoryBuffer, MemoryBufferError},
    trap::InterruptInternal,
    Address,
};

use super::VMState;

pub struct SwiController {
    mode: PrivilegeMode,
    interrupts: IntMap<usize, Rc<Mutex<BitFlags<InterruptInternal>>>>,
    // hart_count: usize,
}

impl SwiController {
    pub(super) fn new(harts: &Vec<Hart>, mode: PrivilegeMode) -> Self {
        let interrupts = harts
            .iter()
            .map(|h| (h.get_hart_id() as usize, h.get_mip_ref()))
            .collect::<IntMap<usize, Rc<Mutex<BitFlags<InterruptInternal>>>>>();

        Self { mode, interrupts }
    }
}

impl MemoryBuffer for SwiController {
    fn size(&self) -> u64 {
        // (self.hart_count as u64) * 8
        (self.interrupts.len() as u64) * 8
    }

    fn write_bytes(&mut self, bytes: &[u8], addr: Address) -> Result<(), MemoryBufferError> {
        dbg!(bytes);
        dbg!(addr);
        let addr = <Address as Into<u64>>::into(addr);
        if addr % 8 != 0 {
            return Err(MemoryBufferError::UnalignedWrite(addr.into()));
        }
        let hartid = addr / 8;

        let bits = self
            .interrupts
            .get(&(hartid as usize))
            .ok_or(MemoryBufferError::OutOfBoundsWrite(addr.into()))?;

        let mut bits = bits.lock().unwrap();

        if bytes[0] % 2 == 1 {
            match self.mode {
                PrivilegeMode::User => unreachable!(),
                PrivilegeMode::Supervisor => *bits |= InterruptInternal::SupervisorSoftware,
                PrivilegeMode::Machine => *bits |= InterruptInternal::MachineSoftware,
            }
        } else {
            match self.mode {
                PrivilegeMode::User => unreachable!(),
                PrivilegeMode::Supervisor => *bits &= !InterruptInternal::SupervisorSoftware,
                PrivilegeMode::Machine => *bits &= !InterruptInternal::MachineSoftware,
            }
        }

        Ok(())
    }

    fn read_bytes(&self, addr: Address, size: usize) -> Result<Vec<u8>, MemoryBufferError> {
        let addr = <Address as Into<u64>>::into(addr);
        if addr % 8 != 0 {
            return Err(MemoryBufferError::UnalignedWrite(addr.into()));
        }
        let hartid = addr / 8;

        let bits = self
            .interrupts
            .get(&(hartid as usize))
            .ok_or(MemoryBufferError::OutOfBoundsWrite(addr.into()))?;

        let bits = bits.lock().unwrap();

        let mut bytes = vec![0u8; size];
        bytes[0] = match self.mode {
            PrivilegeMode::User => unreachable!(),
            PrivilegeMode::Supervisor => bits.contains(InterruptInternal::SupervisorSoftware),
            PrivilegeMode::Machine => bits.contains(InterruptInternal::MachineSoftware),
        } as u8;

        Ok(bytes)
    }
}
