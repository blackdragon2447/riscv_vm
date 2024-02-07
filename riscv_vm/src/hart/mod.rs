mod csr_address;
mod csr_holder;
pub mod isa;
pub mod privilege;
#[cfg(test)]
mod tests;
pub mod trap;
mod counters;

use std::{collections::HashMap, time::Instant, usize};

use crate::{
    decode::{decode, instruction::Instruction},
    execute::{execute_rv64, ExecuteError, ExecuteResult},
    memory::{
        address::Address,
        registers::{IntRegister, Registers},
        Memory, MemoryError,
    },
    vmstate::{VMError, VMSettings},
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
    vm_settings: VMSettings,
}

impl Hart {
    pub fn new(hart_id: u64, vm_settings: VMSettings) -> Self {
        Self {
            hart_id,
            pc: 0x80000000u64.into(),
            registers: Registers::new(),
            csr: CsrHolder::new(hart_id),
            privilege: PrivilegeMode::Machine,
            vm_settings,
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

    pub fn get_csr_mut(&mut self) -> &mut CsrHolder {
        &mut self.csr
    }

    pub fn get_csr(&self) -> &CsrHolder {
        &self.csr
    }

    pub fn privilege(&self) -> PrivilegeMode {
        self.privilege
    }

    pub fn set_privilege(&mut self, privilege: PrivilegeMode) {
        self.privilege = privilege;
    }

    pub fn step(&mut self, mem: &mut Memory, verbose: bool) -> Result<(), VMError> {
        let inst = match self.fetch(mem) {
            Ok(inst) => inst,
            Err(err) => match err {
                MemoryError::PmpDeniedFetch => {
                    return self.exception(Exception::InstructionAccessFault);
                }
                MemoryError::PageFaultFetch => {
                    return self.exception(Exception::InstructionPageFault);
                }
                MemoryError::OutOfBoundsRead(_, _) => {
                    return self.exception(Exception::InstructionAccessFault);
                }
                _ => unreachable!("fetch may not return non fetch errors"),
            },
        };
        if verbose {
            println!("{:#?}", &inst);
        }
        let result = execute_rv64(self, mem, inst, self.csr.isa());
        match result {
            Ok(ExecuteResult::Continue) => self.inc_pc(),
            Ok(ExecuteResult::Jump(pc)) => self.set_pc(pc),
            Ok(ExecuteResult::CsrUpdate(addr)) => {
                if addr == 0x180u16.into() && self.csr.status.tvm {
                    return self.exception(Exception::IllegalInstruction);
                }
                self.inc_pc();
            }
            Err(ExecuteError::Exception(e)) => return self.exception(e),
            Err(ExecuteError::Fatal) => return Err(VMError::ExecureError(ExecuteError::Fatal)),
        };

        self.csr.inc_cycle(1);
        self.csr.inc_instret(1);

        Ok(())
    }

    pub fn step_until(
        &mut self,
        mem: &mut Memory,
        target: Address,
        limit: usize,
    ) -> Result<(), VMError> {
        let mut i = 0;
        while self.pc != target && i < limit {
            i += 1;
            self.step(mem, false)?
        }
        if i >= limit {
            Err(VMError::StepUntilLimit)
        } else {
            Ok(())
        }
    }

    pub fn fetch(&self, mem: &mut Memory) -> Result<Instruction, MemoryError> {
        let window = mem.window(self);
        let inst = window.fetch(self.get_pc())?;
        Ok(decode(inst))
    }

    pub fn pmp_enable(&self) -> bool {
        self.vm_settings.pmp_enable
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
            self.set_pc(self.csr.stvec.base);
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
            self.set_pc(self.csr.mtvec.base);
            Ok(())
        }
    }
}
