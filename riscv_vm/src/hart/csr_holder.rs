use super::{csr_address::CsrType, CsrAddress};
use std::collections::HashMap;

#[derive(Debug)]
pub struct CsrHolder {
    cycle: u64,
    time: u64,
    instret: u64,
    hart_id: u64,
    mtvec: u64,
    csr: HashMap<CsrAddress, u64>,
}

impl CsrHolder {
    pub fn new(hart_id: u64) -> Self {
        Self {
            // UserMode
            cycle: 0,
            time: 0,
            instret: 0,
            // MachineMode
            hart_id,
            mtvec: 0,
            csr: HashMap::new(),
        }
    }

    pub(in crate::hart) fn inc_cycle(&mut self, value: u64) {
        self.cycle += value;
    }
    pub(in crate::hart) fn write_time(&mut self, value: u64) {
        self.time = value;
    }
    pub(in crate::hart) fn inc_instret(&mut self, value: u64) {
        self.instret += value;
    }

    pub fn get_csr(&self, addr: CsrAddress) -> u64 {
        match addr.into() {
            // UserMode
            0xC00u16 => self.cycle,
            0xC01u16 => self.time,
            0xC02u16 => self.instret,
            // MachineMode
            0xF15u16 => self.hart_id,
            0x306u16 => self.mtvec,
            _ => *self.csr.get(&addr).unwrap_or(&0),
        }
    }

    pub fn write_csr(&mut self, addr: CsrAddress, value: u64, should_read: bool) -> Option<u64> {
        if addr.get_type() == CsrType::StandardRO || addr.get_type() == CsrType::CustomRO {
            if should_read {
                Some(self.get_csr(addr))
            } else {
                None
            }
        } else if should_read {
            match addr.into() {
                0x305u16 => {
                    let mode = value & 0b11;
                    if mode < 2 {
                        self.mtvec = value;
                    }
                    Some(self.mtvec)
                }
                _ => {
                    if let Some(csr) = self.csr.get_mut(&addr) {
                        let ret = *csr;
                        *csr = value;
                        Some(ret)
                    } else {
                        self.csr.insert(addr, value);
                        Some(0)
                    }
                }
            }
        } else {
            self.csr.insert(addr, value);
            None
        }
    }

    pub fn set_csr(&mut self, addr: CsrAddress, mask: u64, should_write: bool) -> u64 {
        if addr.get_type() == CsrType::StandardRO || addr.get_type() == CsrType::CustomRO {
            self.get_csr(addr)
        } else if should_write {
            match addr.into() {
                0x305u16 => {
                    let mode = (self.mtvec | mask) & 0b11;
                    if mode < 2 {
                        self.mtvec |= mask;
                    }
                    self.mtvec
                }
                _ => {
                    if let Some(csr) = self.csr.get_mut(&addr) {
                        let ret = *csr;
                        *csr = ret | mask;
                        ret
                    } else {
                        self.csr.insert(addr, mask);
                        0
                    }
                }
            }
        } else {
            match addr.into() {
                0x305u16 => self.mtvec,
                _ => self.csr.get(&addr).copied().unwrap_or_default(),
            }
        }
    }

    pub fn clear_csr(&mut self, addr: CsrAddress, mask: u64, should_write: bool) -> u64 {
        if addr.get_type() == CsrType::StandardRO || addr.get_type() == CsrType::CustomRO {
            self.get_csr(addr)
        } else if should_write {
            match addr.into() {
                0x305u16 => {
                    let mode = (self.mtvec & !mask) & 0b11;
                    if mode < 2 {
                        self.mtvec &= !mask;
                    }
                    self.mtvec
                }
                _ => {
                    if let Some(csr) = self.csr.get_mut(&addr) {
                        let ret = *csr;
                        *csr = ret & !mask;
                        ret
                    } else {
                        self.csr.insert(addr, 0);
                        0
                    }
                }
            }
        } else {
            match addr.into() {
                0x305u16 => self.mtvec,
                _ => self.csr.get(&addr).copied().unwrap_or_default(),
            }
        }
    }
}
