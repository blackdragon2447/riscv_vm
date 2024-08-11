use csrind::MiReg;

use crate::{execute::ExecuteError, trap::Exception};

use super::CsrAddress;

pub(crate) mod csr_address;
pub(crate) mod csr_holder;
pub(crate) mod csrind;
#[cfg(test)]
mod tests;

#[allow(unused)]
pub trait CsrProvider {
    fn has_csr(&self, addr: CsrAddress) -> bool;

    fn get_csr(&self, addr: CsrAddress) -> Option<u64>;

    fn write_csr(
        &mut self,
        addr: CsrAddress,
        value: u64,
        should_read: bool,
    ) -> Result<Option<u64>, ExecuteError>;

    fn set_csr(
        &mut self,
        addr: CsrAddress,
        mask: u64,
        should_write: bool,
    ) -> Result<u64, ExecuteError>;

    fn clear_csr(
        &mut self,
        addr: CsrAddress,
        mask: u64,
        should_write: bool,
    ) -> Result<u64, ExecuteError>;

    fn has_mireg(&self, reg: MiReg, select: u64) -> bool;
    fn has_miselct(&self, select: u64) -> bool;

    fn get_mireg(&self, reg: MiReg, select: u64) -> Option<u64> {
        None
    }

    fn write_mireg(
        &mut self,
        reg: MiReg,
        select: u64,
        value: u64,
        should_read: bool,
    ) -> Result<Option<u64>, ExecuteError> {
        Err(ExecuteError::Exception(Exception::IllegalInstruction))
    }

    fn set_mireg(
        &mut self,
        reg: MiReg,
        select: u64,
        mask: u64,
        should_write: bool,
    ) -> Result<u64, ExecuteError> {
        Err(ExecuteError::Exception(Exception::IllegalInstruction))
    }

    fn clear_mireg(
        &mut self,
        reg: MiReg,
        select: u64,
        mask: u64,
        should_write: bool,
    ) -> Result<u64, ExecuteError> {
        Err(ExecuteError::Exception(Exception::IllegalInstruction))
    }

    fn has_sireg(&self, reg: MiReg, select: u64) -> bool;
    fn has_siselct(&self, select: u64) -> bool;

    fn get_sireg(&self, reg: MiReg, select: u64) -> Option<u64> {
        None
    }

    fn write_sireg(
        &mut self,
        reg: MiReg,
        select: u64,
        value: u64,
        should_read: bool,
    ) -> Result<Option<u64>, ExecuteError> {
        Err(ExecuteError::Exception(Exception::IllegalInstruction))
    }

    fn set_sireg(
        &mut self,
        reg: MiReg,
        select: u64,
        mask: u64,
        should_write: bool,
    ) -> Result<u64, ExecuteError> {
        Err(ExecuteError::Exception(Exception::IllegalInstruction))
    }

    fn clear_sireg(
        &mut self,
        reg: MiReg,
        select: u64,
        mask: u64,
        should_write: bool,
    ) -> Result<u64, ExecuteError> {
        Err(ExecuteError::Exception(Exception::IllegalInstruction))
    }
}
