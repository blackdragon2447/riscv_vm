mod csr_address;
mod csr_holder;
#[cfg(test)]
mod tests;

use std::{collections::HashMap, time::Instant};

use crate::{
    decode::decode,
    execute::execute,
    memory::{
        address::Address,
        registers::{IntRegister, Registers},
        Memory,
    },
    vmstate::VMError,
};

pub use csr_address::CsrAddress;

use self::csr_holder::CsrHolder;

#[derive(Debug)]
pub struct Hart {
    hart_id: u64,
    mode: Mode,
    pc: Address,
    registers: Registers,
    csr: CsrHolder,
    started: Instant,
}

#[derive(Debug)]
pub enum Mode {
    User,
    SuperVisor,
    Machine,
}

impl Hart {
    pub fn new(hart_id: u64) -> Self {
        Self {
            hart_id,
            mode: Mode::Machine,
            pc: 0x80000000u64.into(),
            registers: Registers::new(),
            csr: CsrHolder::new(hart_id),
            started: Instant::now(),
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

    pub fn step<const SIZE: usize>(&mut self, mem: &mut Memory<SIZE>) -> Result<(), VMError> {
        // Unwrap here is safe since u32 expects 4 bytes and we alyaws read 4 bytes (read_bytes
        // will return an Err if it cannot).
        let inst = decode(u32::from_le_bytes(
            mem.read_bytes(self.get_pc(), 4)?.try_into().unwrap(),
        ));
        dbg!(inst);
        execute(self, mem, inst);

        self.csr.inc_cycle(1);
        self.csr
            .write_time(self.started.elapsed().as_millis() as u64);
        self.csr.inc_instret(1);

        Ok(())
    }
}
