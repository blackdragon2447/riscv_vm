use crate::{
    execute::ExecuteError,
    hart::{
        csr::csr_address::{CsrAddress, CsrType},
        privilege::PrivilegeMode,
    },
    interrupt::timer::TimerRef,
    trap::Exception,
};

use super::{csr_holder::CsrHolder, csrind::MiReg, CsrProvider};

#[test]
fn cycle_csr_type() {
    assert_eq!(
        <u16 as Into<CsrAddress>>::into(0xC00).get_type(),
        CsrType::StandardRO
    )
}

#[test]
fn time_csr_type() {
    assert_eq!(
        <u16 as Into<CsrAddress>>::into(0xC01).get_type(),
        CsrType::StandardRO
    )
}

#[test]
fn instret_csr_type() {
    assert_eq!(
        <u16 as Into<CsrAddress>>::into(0xC02).get_type(),
        CsrType::StandardRO
    )
}

#[derive(Default)]
struct TestCsrProvider {
    csrs: [u64; 16],
    miregs: [u64; 16],
    siregs: [u64; 16],
}

impl CsrProvider for TestCsrProvider {
    fn has_csr(&self, addr: CsrAddress) -> bool {
        (0x10..0x1F).contains(&<CsrAddress as Into<u16>>::into(addr))
    }

    fn get_csr(&self, addr: CsrAddress) -> Option<u64> {
        self.csrs
            .get((<CsrAddress as Into<u16>>::into(addr) - 0x10) as usize)
            .copied()
    }

    fn write_csr(
        &mut self,
        addr: CsrAddress,
        value: u64,
        should_read: bool,
    ) -> Result<Option<u64>, ExecuteError> {
        let val = self.get_csr(addr);
        let Some(csr) = self
            .csrs
            .get_mut((<CsrAddress as Into<u16>>::into(addr) - 0x10) as usize)
        else {
            return Err(ExecuteError::Exception(Exception::IllegalInstruction));
        };

        *csr = value;

        if should_read {
            Ok(val)
        } else {
            Ok(None)
        }
    }

    fn set_csr(
        &mut self,
        addr: CsrAddress,
        mask: u64,
        should_write: bool,
    ) -> Result<u64, ExecuteError> {
        let Some(val) = self.get_csr(addr) else {
            return Err(ExecuteError::Exception(Exception::IllegalInstruction));
        };
        let Some(csr) = self
            .csrs
            .get_mut((<CsrAddress as Into<u16>>::into(addr) - 0x10) as usize)
        else {
            return Err(ExecuteError::Exception(Exception::IllegalInstruction));
        };

        if should_write {
            *csr = val | mask;
        }

        Ok(val)
    }

    fn clear_csr(
        &mut self,
        addr: CsrAddress,
        mask: u64,
        should_write: bool,
    ) -> Result<u64, ExecuteError> {
        let Some(val) = self.get_csr(addr) else {
            return Err(ExecuteError::Exception(Exception::IllegalInstruction));
        };
        let Some(csr) = self
            .csrs
            .get_mut((<CsrAddress as Into<u16>>::into(addr) - 0x10) as usize)
        else {
            return Err(ExecuteError::Exception(Exception::IllegalInstruction));
        };

        if should_write {
            *csr = val & !mask;
        }

        Ok(val)
    }

    fn has_mireg(&self, reg: MiReg, select: u64) -> bool {
        reg == MiReg::MiReg1 && (0x0..0xF).contains(&select)
    }

    fn has_miselct(&self, select: u64) -> bool {
        (0x0..0xF).contains(&select)
    }

    fn get_mireg(&self, reg: MiReg, select: u64) -> Option<u64> {
        if reg == MiReg::MiReg1 {
            self.miregs.get(select as usize).cloned()
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
            let val = self.get_mireg(reg, select);
            let Some(csr) = self.miregs.get_mut(select as usize) else {
                return Err(ExecuteError::Exception(Exception::IllegalInstruction));
            };

            *csr = value;

            if should_read {
                Ok(val)
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
            let Some(val) = self.get_mireg(reg, select) else {
                return Err(ExecuteError::Exception(Exception::IllegalInstruction));
            };

            let Some(csr) = self.miregs.get_mut(select as usize) else {
                return Err(ExecuteError::Exception(Exception::IllegalInstruction));
            };

            if should_write {
                *csr = val | mask;
            }

            Ok(val)
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
            let Some(val) = self.get_mireg(reg, select) else {
                return Err(ExecuteError::Exception(Exception::IllegalInstruction));
            };

            let Some(csr) = self.miregs.get_mut(select as usize) else {
                return Err(ExecuteError::Exception(Exception::IllegalInstruction));
            };

            if should_write {
                *csr = val & !mask;
            }

            Ok(val)
        } else {
            Err(ExecuteError::Exception(Exception::IllegalInstruction))
        }
    }

    fn has_sireg(&self, reg: MiReg, select: u64) -> bool {
        reg == MiReg::MiReg1 && (0x0..0xF).contains(&select)
    }

    fn has_siselct(&self, select: u64) -> bool {
        (0x0..0xF).contains(&select)
    }

    fn get_sireg(&self, reg: MiReg, select: u64) -> Option<u64> {
        if reg == MiReg::MiReg1 {
            self.siregs.get(select as usize).cloned()
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
            let val = self.get_sireg(reg, select);
            let Some(csr) = self.siregs.get_mut(select as usize) else {
                return Err(ExecuteError::Exception(Exception::IllegalInstruction));
            };

            *csr = value;

            if should_read {
                Ok(val)
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
            let Some(val) = self.get_sireg(reg, select) else {
                return Err(ExecuteError::Exception(Exception::IllegalInstruction));
            };

            let Some(csr) = self.siregs.get_mut(select as usize) else {
                return Err(ExecuteError::Exception(Exception::IllegalInstruction));
            };

            if should_write {
                *csr = val | mask;
            }

            Ok(val)
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
            let Some(val) = self.get_sireg(reg, select) else {
                return Err(ExecuteError::Exception(Exception::IllegalInstruction));
            };

            let Some(csr) = self.siregs.get_mut(select as usize) else {
                return Err(ExecuteError::Exception(Exception::IllegalInstruction));
            };

            if should_write {
                *csr = val & !mask;
            }

            Ok(val)
        } else {
            Err(ExecuteError::Exception(Exception::IllegalInstruction))
        }
    }
}

#[test]
fn get_csr_holder() {
    let mut csr = CsrHolder::new(37, TimerRef::dummy());

    csr.add_csr_provider(TestCsrProvider::default());

    assert_eq!(csr.get_csr(0xF14u16.into()), Some(37));

    assert_eq!(
        csr.write_csr(0x14u16.into(), 42, PrivilegeMode::Machine, true),
        Ok(Some(0))
    );

    assert_eq!(
        csr.write_csr(0x150u16.into(), 0xA, PrivilegeMode::Supervisor, false),
        Ok(None)
    );

    assert_eq!(
        csr.write_csr(0x151u16.into(), 144, PrivilegeMode::Supervisor, true),
        Ok(Some(0))
    );

    assert_eq!(
        csr.write_csr(0x150u16.into(), 0xB, PrivilegeMode::Supervisor, false),
        Ok(None)
    );

    assert_eq!(
        csr.set_csr(0x151u16.into(), 0, PrivilegeMode::Supervisor, false),
        Ok(0)
    );

    assert_eq!(
        csr.write_csr(0x150u16.into(), 0xA, PrivilegeMode::Supervisor, false),
        Ok(None)
    );

    assert_eq!(
        csr.set_csr(0x151u16.into(), 0, PrivilegeMode::Supervisor, false),
        Ok(144)
    );
}
