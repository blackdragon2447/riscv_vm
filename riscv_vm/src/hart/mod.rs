mod csr_address;
mod csr_holder;
pub mod isa;
pub mod privilege;
#[cfg(test)]
mod tests;
pub mod trap;

use std::{collections::HashMap, time::Instant};

use crate::{
    decode::{decode, instruction::Instruction},
    execute::{execute_rv64, ExecuteError, ExecuteResult},
    memory::{
        address::Address,
        registers::{IntRegister, Registers},
        Memory, MemoryError,
    },
    vmstate::VMError,
};

pub use csr_address::CsrAddress;

use self::{csr_holder::CsrHolder, privilege::PrivilegeMode, trap::Exception};

#[derive(Debug)]
pub struct Hart {
    hart_id: u64,
    pc: Address,
    registers: Registers,
    csr: CsrHolder,
    privilege: PrivilegeMode,
}

impl Hart {
    pub fn new(hart_id: u64) -> Self {
        Self {
            hart_id,
            pc: 0x80000000u64.into(),
            registers: Registers::new(),
            csr: CsrHolder::new(hart_id),
            privilege: PrivilegeMode::Machine,
        }
    }

    pub fn get_hart_id(&self) -> u64 {
        self.hart_id
    }

    pub fn get_pc(&self) -> Address {
        self.pc
    }

    pub fn inc_pc(&mut self) {
        self.pc += 4;
    }

    pub fn set_pc(&mut self, pc: Address) {
        self.pc = pc;
    }

    pub fn get_reg(&self, register: IntRegister) -> i64 {
        self.registers.get(register)
    }

    pub fn set_reg(&mut self, register: IntRegister, value: i64) {
        self.registers.set(register, value)
    }

    pub fn get_csr(&mut self) -> &mut CsrHolder {
        &mut self.csr
    }

    pub fn privilege(&self) -> PrivilegeMode {
        self.privilege
    }

    pub fn set_privilege(&mut self, privilege: PrivilegeMode) {
        self.privilege = privilege;
    }

    pub fn step<const SIZE: usize>(&mut self, mem: &mut Memory<SIZE>) -> Result<(), VMError> {
        // Unwrap here is safe since u32 expects 4 bytes and we alyaws read 4 bytes (read_bytes
        // will return an Err if it cannot).
        let Ok(inst) = self.fetch(mem) else {
            return self.exception(Exception::InstructionAccessFault);
        };
        let result = execute_rv64(self, mem, inst, self.csr.isa());
        match result {
            Ok(ExecuteResult::Continue) => self.inc_pc(),
            Ok(ExecuteResult::Jump(pc)) => self.set_pc(pc),
            Err(ExecuteError::Exception(e)) => return self.exception(e),
            Err(ExecuteError::Fatal) => return Err(VMError::ExecureError(ExecuteError::Fatal)),
        };

        self.csr.inc_cycle(1);
        self.csr.inc_instret(1);

        Ok(())
    }

    fn fetch<const SIZE: usize>(&self, mem: &Memory<SIZE>) -> Result<Instruction, MemoryError> {
        let inst_bytes = mem.read_bytes(self.get_pc(), 4)?;
        let inst = decode(u32::from_le_bytes(inst_bytes.try_into().unwrap()));
        Ok(inst)
    }

    fn exception(&mut self, exception: Exception) -> Result<(), VMError> {
        eprintln!("Exeption hit: {:?} ({:?})", exception, exception.get_code());
        if self.csr.medeleg.contains(exception) {
            eprintln!("Delegating exception to S mode");
            self.csr.scause = exception.get_code();
            self.csr.sepc = self.get_pc();
            if 8 <= (exception.get_code()) && (exception.get_code()) <= 11 {
                // ECALL
            } else {
                self.csr.inc_cycle(1);
                self.csr.inc_instret(1);
            }
            self.csr.status.spie = self.csr.status.sie;
            self.csr.status.sie = false;
            self.csr.status.spp = self.privilege();
            self.privilege = PrivilegeMode::Supervisor;
            self.set_pc(self.csr.stvec);
            Ok(())
        } else {
            eprintln!("Delegating exception to M mode");
            self.csr.mcause = exception.get_code();
            self.csr.mepc = self.get_pc();
            if 8 <= (exception.get_code()) && (exception.get_code()) <= 11 {
                // ECALL
            } else {
                self.csr.inc_cycle(1);
                self.csr.inc_instret(1);
            }
            self.csr.status.mpie = self.csr.status.mie;
            self.csr.status.mie = false;
            self.csr.status.mpp = self.privilege();
            self.privilege = PrivilegeMode::Machine;
            self.set_pc(self.csr.mtvec);
            Ok(())
        }
    }
}
