mod counters;
mod csr_address;
mod csr_holder;
pub mod isa;
pub mod privilege;
pub mod registers;
#[cfg(test)]
mod tests;
pub mod trap;

use core::panic;
#[cfg(feature = "float")]
use softfloat_wrapper::{F32, F64};
use std::{
    collections::{BinaryHeap, HashMap},
    rc::Rc,
    sync::Mutex,
    time::Instant,
    usize,
};

use crate::{
    decode::{decode, Instruction},
    execute::{execute_rv64, ExecuteError, ExecuteResult},
    hart::csr_holder::TrapMode,
    memory::{address::Address, Memory, MemoryError},
    vmstate::{timer::TimerRef, VMError, VMSettings},
};

pub use csr_address::CsrAddress;
use enumflags2::BitFlags;
#[cfg(feature = "float")]
use registers::{FloatRegister, InvalidNaNBox};

use self::{
    csr_holder::CsrHolder,
    privilege::PrivilegeMode,
    registers::{IntRegister, Registers},
    trap::{Exception, Interrupt, InterruptInternal, TrapCause},
};

#[derive(Debug)]
pub struct Hart {
    hart_id: u64,
    pc: Address,
    registers: Registers,
    csr: CsrHolder,
    privilege: PrivilegeMode,
    vm_settings: VMSettings,
    waiting_for_interrupt: bool,
}

impl Hart {
    pub fn new(hart_id: u64, vm_settings: VMSettings, timer: TimerRef) -> Self {
        Self {
            hart_id,
            pc: 0x80000000u64.into(),
            registers: Registers::new(),
            csr: CsrHolder::new(hart_id, timer),
            privilege: PrivilegeMode::Machine,
            vm_settings,
            waiting_for_interrupt: false,
        }
    }

    pub fn get_hart_id(&self) -> u64 {
        self.hart_id
    }

    pub fn get_pc(&self) -> Address {
        self.pc
    }

    pub fn inc_pc(&mut self, if_compact: bool) {
        if if_compact {
            self.pc += 2;
        } else {
            self.pc += 4;
        }
    }

    pub fn set_pc(&mut self, pc: Address) {
        self.pc = pc;
    }

    pub fn get_int_reg(&self, register: IntRegister) -> i64 {
        self.registers.get_int(register)
    }

    pub fn set_int_reg(&mut self, register: IntRegister, value: i64) {
        self.registers.set_int(register, value)
    }

    #[cfg(feature = "float")]
    pub fn get_f32_reg(&self, register: FloatRegister) -> Result<F32, InvalidNaNBox> {
        self.registers.get_f32(register)
    }

    #[cfg(feature = "float")]
    pub fn set_f32_reg(&mut self, register: FloatRegister, value: F32) {
        self.registers.set_f32(register, value)
    }

    #[cfg(feature = "float")]
    pub fn get_f64_reg(&self, register: FloatRegister) -> F64 {
        self.registers.get_f64(register)
    }

    #[cfg(feature = "float")]
    pub fn set_f64_reg(&mut self, register: FloatRegister, value: F64) {
        self.registers.set_f64(register, value)
    }

    pub fn get_csr_mut(&mut self) -> &mut CsrHolder {
        &mut self.csr
    }

    pub fn get_csr(&self) -> &CsrHolder {
        &self.csr
    }

    pub fn get_mip_ref(&self) -> Rc<Mutex<BitFlags<InterruptInternal>>> {
        self.csr.mip.clone()
    }

    pub fn privilege(&self) -> PrivilegeMode {
        self.privilege
    }

    pub fn set_privilege(&mut self, privilege: PrivilegeMode) {
        self.privilege = privilege;
    }

    pub fn wait_for_interrupt(&mut self) {
        self.waiting_for_interrupt = true;
    }

    pub fn step(&mut self, mem: &mut Memory, verbose: bool) -> Result<(), VMError> {
        let mip_ref = self.get_mip_ref();
        let mip = mip_ref.lock().unwrap();
        if mip.bits() != 0 {
            'interrupt_loop: for i in mip
                .iter()
                .collect::<BinaryHeap<InterruptInternal>>()
                .into_sorted_vec()
            {
                match i {
                    InterruptInternal::SupervisorSoftware
                    | InterruptInternal::SupervisorTimer
                    | InterruptInternal::SupervisorExternal => {
                        if self.csr.mideleg.contains(i) {
                            if (self.privilege < PrivilegeMode::Supervisor
                                || (self.privilege == PrivilegeMode::Supervisor
                                    && self.csr.status.sie))
                                && self.csr.sie.contains(i)
                            {
                                self.trap(TrapCause::Interrupt(i), PrivilegeMode::Supervisor);
                                break 'interrupt_loop;
                            }
                        } else if (self.privilege < PrivilegeMode::Machine || self.csr.status.mie)
                            && self.csr.mie.contains(i)
                        {
                            self.trap(TrapCause::Interrupt(i), PrivilegeMode::Machine);
                            break 'interrupt_loop;
                        }
                    }
                    InterruptInternal::MachineSoftware
                    | InterruptInternal::MachineTimer
                    | InterruptInternal::MachineExternal => {
                        if (self.privilege < PrivilegeMode::Machine || self.csr.status.mie)
                            && self.csr.mie.contains(i)
                        {
                            self.trap(TrapCause::Interrupt(i), PrivilegeMode::Machine);
                            break 'interrupt_loop;
                        }
                    }
                }
            }
        }
        drop(mip);
        drop(mip_ref);

        if (self.waiting_for_interrupt) {
            return Ok(());
        }

        let (inst, is_compact) = match self.fetch(mem) {
            Ok(inst) => inst,
            Err(err) => match err {
                MemoryError::PmpDeniedFetch => {
                    self.exception(Exception::InstructionAccessFault);
                    return Ok(());
                }
                MemoryError::PageFaultFetch => {
                    self.exception(Exception::InstructionPageFault);
                    return Ok(());
                }
                MemoryError::OutOfBoundsRead(_) => {
                    self.exception(Exception::InstructionAccessFault);
                    return Ok(());
                }
                _ => unreachable!("fetch may not return non fetch errors"),
            },
        };

        if verbose {
            println!("{:#?}", &inst);
        }

        let result = execute_rv64(self, mem, inst, is_compact, self.csr.isa());
        match result {
            Ok(ExecuteResult::Continue) => self.inc_pc(is_compact),
            Ok(ExecuteResult::WFI) => self.wait_for_interrupt(),
            Ok(ExecuteResult::Jump(pc)) => self.set_pc(pc),
            Ok(ExecuteResult::CsrUpdate(addr)) => {
                if addr == 0x180u16.into() && self.csr.status.tvm {
                    self.exception(Exception::IllegalInstruction);
                    return Ok(());
                }
                self.inc_pc(is_compact);
            }
            Err(ExecuteError::Exception(e)) => {
                self.exception(e);
                return Ok(());
            }
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

    pub fn fetch(&self, mem: &mut Memory) -> Result<(Instruction, bool), MemoryError> {
        let mut window = mem.window(self);
        let inst = window.fetch(self.get_pc())?;
        Ok(decode(inst))
    }

    pub fn pmp_enable(&self) -> bool {
        self.vm_settings.pmp_enable
    }

    fn exception(&mut self, exception: Exception) {
        eprintln!("Exeption hit: {:?} ({:?})", exception, exception.get_code());
        if self.csr.medeleg.contains(exception) && self.privilege < PrivilegeMode::Machine {
            eprintln!("Delegating exception to S mode");
            self.trap(TrapCause::Exception(exception), PrivilegeMode::Supervisor);
        } else {
            eprintln!("Delegating exception to M mode");
            self.trap(TrapCause::Exception(exception), PrivilegeMode::Machine);
        }
    }

    fn trap(&mut self, cause: TrapCause, target: PrivilegeMode) {
        self.waiting_for_interrupt = false;
        match target {
            PrivilegeMode::User => unreachable!("User mode cannot handle traps"),
            PrivilegeMode::Supervisor => {
                self.csr.sepc = self.get_pc();
                self.csr.status.spie = self.csr.status.sie;
                self.csr.status.sie = false;
                self.csr.status.spp = self.privilege();
                self.privilege = PrivilegeMode::Supervisor;
                match cause {
                    TrapCause::Exception(e) => {
                        if 8 <= (e.get_code()) && (e.get_code()) <= 11 {
                            // ECALL
                        } else {
                            self.csr.inc_cycle(1);
                            self.csr.inc_instret(1);
                        }
                        self.csr.scause = e.get_code();
                        self.set_pc(self.csr.stvec.base);
                    }
                    TrapCause::Interrupt(i) => {
                        self.csr.inc_cycle(1);
                        self.csr.inc_instret(1);
                        self.csr.scause = i.get_code();
                        self.set_pc(self.csr.stvec.base + 4 * i.get_code());
                    }
                }
            }
            PrivilegeMode::Machine => {
                self.csr.mepc = self.get_pc();
                self.csr.status.mpie = self.csr.status.mie;
                self.csr.status.mie = false;
                self.csr.status.mpp = self.privilege();
                self.privilege = PrivilegeMode::Machine;
                match cause {
                    TrapCause::Exception(e) => {
                        if 8 <= (e.get_code()) && (e.get_code()) <= 11 {
                            // ECALL
                        } else {
                            self.csr.inc_cycle(1);
                            self.csr.inc_instret(1);
                        }
                        self.csr.mcause = e.get_code();
                        self.set_pc(self.csr.mtvec.base);
                    }
                    TrapCause::Interrupt(i) => {
                        self.csr.inc_cycle(1);
                        self.csr.inc_instret(1);
                        self.csr.mcause = i.get_code();
                        self.csr.mcause |= 0x1 << 63;
                        if self.csr.mtvec.mode == TrapMode::Direct {
                            self.set_pc(self.csr.mtvec.base);
                        } else {
                            self.set_pc(self.csr.mtvec.base + 4 * i.get_code());
                        }
                    }
                }
            }
        }
    }
}
