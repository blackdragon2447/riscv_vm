use crate::hart::privilege::PrivilegeMode;

use super::{address::Address, Memory};

pub struct Register {
    value: u64,
    permission: PrivilegeMode,
}

impl Register {
    pub(super) fn new(permission: PrivilegeMode) -> Self {
        Self {
            value: 0,
            permission,
        }
    }

    pub fn set(&mut self, value: u64) {
        self.value = value;
    }
}

pub struct MemoryRegisterHandle<'a> {
    pub(super) memory_ref: &'a mut Memory,
    pub(super) dev_id: usize,
}

impl<'a> MemoryRegisterHandle<'a> {
    pub fn new(memory: &'a mut Memory, dev_id: usize) -> MemoryRegisterHandle<'a> {
        Self {
            memory_ref: memory,
            dev_id,
        }
    }
}
