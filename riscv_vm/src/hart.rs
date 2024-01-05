use crate::memory::{
    registers::{IntRegister, Registers},
    Address,
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
            pc: 0x80000000.into(),
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
        self.pc += 4.into();
    }
}
