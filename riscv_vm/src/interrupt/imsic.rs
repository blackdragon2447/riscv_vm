use std::{cell::RefCell, rc::Rc, sync::Mutex, usize};

use enumflags2::BitFlags;

use crate::{
    execute::ExecuteError,
    hart::{privilege::PrivilegeMode, CsrAddress, CsrProvider, MiReg},
    memory::memory_buffer::{MemoryBuffer, MemoryBufferError},
    trap::{Exception, InterruptInternal},
    Address,
};

#[derive(Debug)]
pub struct Imsic {
    machine_eidelivery: bool,
    machine_eithreshold: u32,
    machine_pending_file: [u64; 32],
    machine_enable_file: [u64; 32],

    supervisor_eidelivery: bool,
    supervisor_eithreshold: u32,
    supervisor_pending_file: [u64; 32],
    supervisor_enable_file: [u64; 32],

    mip: Rc<Mutex<BitFlags<InterruptInternal>>>,
}

impl Imsic {
    pub fn new(mip: Rc<Mutex<BitFlags<InterruptInternal>>>) -> Self {
        Self {
            machine_eidelivery: false,
            machine_eithreshold: 0,
            machine_pending_file: [0; 32],
            machine_enable_file: [0; 32],
            supervisor_eidelivery: false,
            supervisor_eithreshold: 0,
            supervisor_pending_file: [0; 32],
            supervisor_enable_file: [0; 32],
            mip,
        }
    }

    pub fn m_seteipnum(&mut self, num: u32) {
        if 0 < num && num < 2048 {
            let page = num / 64;
            let bit = num % 64;
            self.machine_pending_file[page as usize] |= 1 << bit;
        }
        self.update_mip();
    }

    pub fn s_seteipnum(&mut self, num: u32) {
        if 0 < num && num < 2048 {
            let page = num / 64;
            let bit = num % 64;
            self.supervisor_pending_file[page as usize] |= 1 << bit;
        }
        self.update_mip();
    }

    pub fn m_cleareipnum(&mut self, num: u32) {
        if 0 < num && num < 2048 {
            let page = num / 64;
            let bit = num % 64;
            self.machine_pending_file[page as usize] &= !(1 << bit);
        }
    }

    pub fn s_cleareipnum(&mut self, num: u32) {
        if 0 < num && num < 2048 {
            let page = num / 64;
            let bit = num % 64;
            self.supervisor_pending_file[page as usize] &= !(1 << bit);
        }
    }

    fn get_mtopei(&self) -> u64 {
        let threshold = if self.machine_eithreshold == 0 {
            2048
        } else {
            self.machine_eithreshold
        };

        for i in 1..threshold {
            if bit_set(&self.machine_pending_file, i) && bit_set(&self.machine_enable_file, i) {
                return i as u64;
            }
        }

        0
    }

    fn get_stopei(&self) -> u64 {
        let threshold = if self.supervisor_eithreshold == 0 {
            2048
        } else {
            self.supervisor_eithreshold
        };

        for i in 1..threshold {
            if bit_set(&self.supervisor_pending_file, i) && bit_set(&self.machine_enable_file, i) {
                return i as u64;
            }
        }

        0
    }

    fn update_mip(&self) {
        self.mip
            .lock()
            .unwrap()
            .set(InterruptInternal::MachineExternal, self.get_mtopei() != 0);
        self.mip.lock().unwrap().set(
            InterruptInternal::SupervisorExternal,
            self.get_stopei() != 0,
        );
    }
}

fn bit_set(file: &[u64; 32], bit: u32) -> bool {
    let page = bit / 64;
    let bit = bit % 64;
    (file[page as usize] >> bit) & 0b1 == 1
}

pub struct ImsicPage {
    imsic: Rc<RefCell<Imsic>>,
    mode: PrivilegeMode,
}

impl ImsicPage {
    fn new(mode: PrivilegeMode, imsic: Rc<RefCell<Imsic>>) -> Self {
        assert!(matches!(
            mode,
            PrivilegeMode::Machine | PrivilegeMode::Supervisor
        ));

        Self { imsic, mode }
    }

    pub fn make_pages(imsic: Rc<RefCell<Imsic>>) -> (ImsicPage, ImsicPage) {
        (
            ImsicPage::new(PrivilegeMode::Machine, imsic.clone()),
            ImsicPage::new(PrivilegeMode::Supervisor, imsic.clone()),
        )
    }
}

impl MemoryBuffer for ImsicPage {
    fn size(&self) -> u64 {
        4096
    }

    fn write_bytes(&mut self, bytes: &[u8], addr: Address) -> Result<(), MemoryBufferError> {
        if addr % 0x4u64 != 0 {
            return Err(MemoryBufferError::UnalignedWrite(addr));
        }
        if addr == 0x0u64.into() {
            let mut buf = [0; 4];
            buf.copy_from_slice(bytes);
            if self.mode == PrivilegeMode::Supervisor {
                RefCell::borrow_mut(&self.imsic).s_seteipnum(u32::from_le_bytes(buf));
            } else if self.mode == PrivilegeMode::Machine {
                RefCell::borrow_mut(&self.imsic).m_seteipnum(u32::from_le_bytes(buf));
            }
        }

        Ok(())
    }

    fn read_bytes(&self, addr: Address, size: usize) -> Result<Vec<u8>, MemoryBufferError> {
        if addr % 0x4u64 != 0 {
            return Err(MemoryBufferError::UnalignedRead(addr));
        }

        Ok(vec![0u8; size])
    }
}

impl CsrProvider for Imsic {
    fn has_csr(&self, addr: CsrAddress) -> bool {
        matches!(<CsrAddress as Into<u16>>::into(addr), 0x15C | 0x35C)
    }

    fn get_csr(&self, addr: CsrAddress) -> Option<u64> {
        match <CsrAddress as Into<u16>>::into(addr) {
            0x15C => {
                if self.supervisor_eidelivery {
                    let id = self.get_stopei();
                    Some((id << 16) | id)
                } else {
                    Some(0)
                }
            }
            0x35C => {
                if self.machine_eidelivery {
                    let id = self.get_mtopei();
                    Some((id << 16) | id)
                } else {
                    Some(0)
                }
            }
            _ => None,
        }
    }

    fn write_csr(
        &mut self,
        addr: CsrAddress,
        _value: u64,
        should_read: bool,
    ) -> Result<Option<u64>, ExecuteError> {
        let Some(old) = self.get_csr(addr) else {
            return Err(ExecuteError::Exception(Exception::IllegalInstruction));
        };

        match <CsrAddress as Into<u16>>::into(addr) {
            0x15C => {
                self.s_cleareipnum((old as u32) >> 16);
            }
            0x35C => {
                self.m_cleareipnum((old as u32) >> 16);
            }
            _ => {
                return Err(ExecuteError::Exception(Exception::IllegalInstruction));
            }
        };

        self.update_mip();

        if should_read {
            Ok(Some(old))
        } else {
            Ok(None)
        }
    }

    fn set_csr(
        &mut self,
        addr: CsrAddress,
        _mask: u64,
        should_write: bool,
    ) -> Result<u64, ExecuteError> {
        let Some(old) = self.get_csr(addr) else {
            return Err(ExecuteError::Exception(Exception::IllegalInstruction));
        };

        if should_write {
            match <CsrAddress as Into<u16>>::into(addr) {
                0x15C => {
                    self.s_cleareipnum((old as u32) >> 16);
                }
                0x35C => {
                    self.m_cleareipnum((old as u32) >> 16);
                }
                _ => {
                    return Err(ExecuteError::Exception(Exception::IllegalInstruction));
                }
            };

            self.update_mip();
        }

        Ok(old)
    }

    fn clear_csr(
        &mut self,
        addr: CsrAddress,
        _mask: u64,
        should_write: bool,
    ) -> Result<u64, ExecuteError> {
        let Some(old) = self.get_csr(addr) else {
            return Err(ExecuteError::Exception(Exception::IllegalInstruction));
        };

        if should_write {
            match <CsrAddress as Into<u16>>::into(addr) {
                0x15C => {
                    self.s_cleareipnum((old as u32) >> 16);
                }
                0x35C => {
                    self.m_cleareipnum((old as u32) >> 16);
                }
                _ => {
                    return Err(ExecuteError::Exception(Exception::IllegalInstruction));
                }
            };

            self.update_mip();
        }

        Ok(old)
    }

    fn has_mireg(&self, reg: MiReg, select: u64) -> bool {
        reg == MiReg::MiReg1 && (0x70..=0xFF).contains(&select)
    }

    fn has_miselct(&self, select: u64) -> bool {
        (0x70..=0xFF).contains(&select)
    }

    fn has_sireg(&self, reg: MiReg, select: u64) -> bool {
        reg == MiReg::MiReg1 && (0x70..=0xFF).contains(&select)
    }

    fn has_siselct(&self, select: u64) -> bool {
        (0x70..=0xFF).contains(&select)
    }

    fn get_mireg(&self, reg: MiReg, select: u64) -> Option<u64> {
        if reg == MiReg::MiReg1 {
            match select {
                0x70 => Some(self.machine_eidelivery as u64),
                0x72 => Some(self.machine_eithreshold as u64),
                i @ 0x80..=0xBF if i % 2 == 0 => {
                    let page = (i - 0x80) / 2;
                    Some(self.machine_pending_file[page as usize])
                }
                i @ 0xC0..=0xFF if i % 2 == 0 => {
                    let page = (i - 0xC0) / 2;
                    Some(self.machine_enable_file[page as usize])
                }
                _ => None,
            }
        } else {
            None
        }
    }

    fn write_mireg(
        &mut self,
        reg: MiReg,
        select: u64,
        value: u64,
        should_read: bool,
    ) -> Result<Option<u64>, ExecuteError> {
        if reg == MiReg::MiReg1 {
            let Some(old) = self.get_mireg(reg, select) else {
                return Err(ExecuteError::Exception(Exception::IllegalInstruction));
            };

            match select {
                0x70 => self.machine_eidelivery = (value & 0b1) != 0,
                0x72 => {
                    if value < 2048 {
                        self.machine_eithreshold = value as u32;
                    }
                }
                i @ 0x80..=0xBF if i % 2 == 0 => {
                    let page = (i - 0x80) / 2;
                    self.machine_pending_file[page as usize] = value;
                }
                i @ 0xC0..=0xFF if i % 2 == 0 => {
                    let page = (i - 0xC0) / 2;
                    self.machine_enable_file[page as usize] = value;
                }
                _ => return Err(ExecuteError::Exception(Exception::IllegalInstruction)),
            }

            self.update_mip();

            if should_read {
                Ok(Some(old))
            } else {
                Ok(None)
            }
        } else {
            Err(ExecuteError::Exception(Exception::IllegalInstruction))
        }
    }

    fn set_mireg(
        &mut self,
        reg: MiReg,
        select: u64,
        mask: u64,
        should_write: bool,
    ) -> Result<u64, ExecuteError> {
        if reg == MiReg::MiReg1 {
            let Some(old) = self.get_mireg(reg, select) else {
                return Err(ExecuteError::Exception(Exception::IllegalInstruction));
            };

            if should_write {
                match select {
                    0x70 => {
                        self.machine_eidelivery =
                            ((self.machine_eidelivery as u64 | mask) & 0b1) != 0
                    }
                    0x72 => {
                        let value = self.machine_eithreshold as u64 | mask;
                        if value < 2048 {
                            self.machine_eithreshold = value as u32;
                        }
                    }
                    i @ 0x80..=0xBF if i % 2 == 0 => {
                        let page = (i - 0x80) / 2;
                        let value = self.machine_pending_file[page as usize] | mask;
                        self.machine_pending_file[page as usize] = value;
                    }
                    i @ 0xC0..=0xFF if i % 2 == 0 => {
                        let page = (i - 0xC0) / 2;
                        let value = self.machine_enable_file[page as usize] | mask;
                        self.machine_enable_file[page as usize] = value;
                    }
                    _ => return Err(ExecuteError::Exception(Exception::IllegalInstruction)),
                }

                self.update_mip();
            }

            Ok(old)
        } else {
            Err(ExecuteError::Exception(Exception::IllegalInstruction))
        }
    }

    fn clear_mireg(
        &mut self,
        reg: MiReg,
        select: u64,
        mask: u64,
        should_write: bool,
    ) -> Result<u64, ExecuteError> {
        if reg == MiReg::MiReg1 {
            let Some(old) = self.get_mireg(reg, select) else {
                return Err(ExecuteError::Exception(Exception::IllegalInstruction));
            };

            if should_write {
                match select {
                    0x70 => {
                        self.machine_eidelivery =
                            ((self.machine_eidelivery as u64 & !mask) & 0b1) != 0
                    }
                    0x72 => {
                        let value = self.machine_eithreshold as u64 & !mask;
                        if value < 2048 {
                            self.machine_eithreshold = value as u32;
                        }
                    }
                    i @ 0x80..=0xBF if i % 2 == 0 => {
                        let page = (i - 0x80) / 2;
                        let value = self.machine_pending_file[page as usize] & !mask;
                        self.machine_pending_file[page as usize] = value;
                    }
                    i @ 0xC0..=0xFF if i % 2 == 0 => {
                        let page = (i - 0xC0) / 2;
                        let value = self.machine_enable_file[page as usize] & !mask;
                        self.machine_enable_file[page as usize] = value;
                    }
                    _ => return Err(ExecuteError::Exception(Exception::IllegalInstruction)),
                }

                self.update_mip();
            }

            Ok(old)
        } else {
            Err(ExecuteError::Exception(Exception::IllegalInstruction))
        }
    }

    fn get_sireg(&self, reg: MiReg, select: u64) -> Option<u64> {
        if reg == MiReg::MiReg1 {
            match select {
                0x70 => Some(self.supervisor_eidelivery as u64),
                0x72 => Some(self.supervisor_eithreshold as u64),
                i @ 0x80..=0xBF if i % 2 == 0 => {
                    let page = (i - 0x80) / 2;
                    Some(self.supervisor_pending_file[page as usize])
                }
                i @ 0xC0..=0xFF if i % 2 == 0 => {
                    let page = (i - 0xC0) / 2;
                    Some(self.supervisor_enable_file[page as usize])
                }
                _ => None,
            }
        } else {
            None
        }
    }

    fn write_sireg(
        &mut self,
        reg: MiReg,
        select: u64,
        value: u64,
        should_read: bool,
    ) -> Result<Option<u64>, ExecuteError> {
        if reg == MiReg::MiReg1 {
            let Some(old) = self.get_mireg(reg, select) else {
                return Err(ExecuteError::Exception(Exception::IllegalInstruction));
            };

            match select {
                0x70 => self.supervisor_eidelivery = (value & 0b1) != 0,
                0x72 => {
                    if value < 2048 {
                        self.supervisor_eithreshold = value as u32;
                    }
                }
                i @ 0x80..=0xBF if i % 2 == 0 => {
                    let page = (i - 0x80) / 2;
                    self.supervisor_pending_file[page as usize] = value;
                }
                i @ 0xC0..=0xFF if i % 2 == 0 => {
                    let page = (i - 0xC0) / 2;
                    self.supervisor_enable_file[page as usize] = value;
                }
                _ => return Err(ExecuteError::Exception(Exception::IllegalInstruction)),
            }

            self.update_mip();

            if should_read {
                Ok(Some(old))
            } else {
                Ok(None)
            }
        } else {
            Err(ExecuteError::Exception(Exception::IllegalInstruction))
        }
    }

    fn set_sireg(
        &mut self,
        reg: MiReg,
        select: u64,
        mask: u64,
        should_write: bool,
    ) -> Result<u64, ExecuteError> {
        if reg == MiReg::MiReg1 {
            let Some(old) = self.get_mireg(reg, select) else {
                return Err(ExecuteError::Exception(Exception::IllegalInstruction));
            };

            if should_write {
                match select {
                    0x70 => {
                        self.supervisor_eidelivery =
                            ((self.supervisor_eidelivery as u64 | mask) & 0b1) != 0
                    }
                    0x72 => {
                        let value = self.supervisor_eithreshold as u64 | mask;
                        if value < 2048 {
                            self.supervisor_eithreshold = value as u32;
                        }
                    }
                    i @ 0x80..=0xBF if i % 2 == 0 => {
                        let page = (i - 0x80) / 2;
                        let value = self.supervisor_pending_file[page as usize] | mask;
                        self.supervisor_pending_file[page as usize] = value;
                    }
                    i @ 0xC0..=0xFF if i % 2 == 0 => {
                        let page = (i - 0xC0) / 2;
                        let value = self.supervisor_enable_file[page as usize] | mask;
                        self.supervisor_enable_file[page as usize] = value;
                    }
                    _ => return Err(ExecuteError::Exception(Exception::IllegalInstruction)),
                }

                self.update_mip();
            }

            Ok(old)
        } else {
            Err(ExecuteError::Exception(Exception::IllegalInstruction))
        }
    }

    fn clear_sireg(
        &mut self,
        reg: MiReg,
        select: u64,
        mask: u64,
        should_write: bool,
    ) -> Result<u64, ExecuteError> {
        if reg == MiReg::MiReg1 {
            let Some(old) = self.get_mireg(reg, select) else {
                return Err(ExecuteError::Exception(Exception::IllegalInstruction));
            };

            if should_write {
                match select {
                    0x70 => {
                        self.supervisor_eidelivery =
                            ((self.supervisor_eidelivery as u64 & !mask) & 0b1) != 0
                    }
                    0x72 => {
                        let value = self.supervisor_eithreshold as u64 & !mask;
                        if value < 2048 {
                            self.supervisor_eithreshold = value as u32;
                        }
                    }
                    i @ 0x80..=0xBF if i % 2 == 0 => {
                        let page = (i - 0x80) / 2;
                        let value = self.supervisor_pending_file[page as usize] & !mask;
                        self.supervisor_pending_file[page as usize] = value;
                    }
                    i @ 0xC0..=0xFF if i % 2 == 0 => {
                        let page = (i - 0xC0) / 2;
                        let value = self.supervisor_enable_file[page as usize] & !mask;
                        self.supervisor_enable_file[page as usize] = value;
                    }
                    _ => return Err(ExecuteError::Exception(Exception::IllegalInstruction)),
                }

                self.update_mip();
            }

            Ok(old)
        } else {
            Err(ExecuteError::Exception(Exception::IllegalInstruction))
        }
    }
}
