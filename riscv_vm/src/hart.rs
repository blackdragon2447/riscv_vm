use crate::memory::{
    address::Address,
    registers::{IntRegister, Registers},
};

#[derive(Debug)]
pub struct Hart {
    hart_id: u64,
    pc: Address,
    registers: Registers,
}

impl Hart {
    pub fn new(hart_id: u64) -> Self {
        Self {
            hart_id,
            pc: 0x80000000u64.into(),
            registers: Registers::new(),
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
}
