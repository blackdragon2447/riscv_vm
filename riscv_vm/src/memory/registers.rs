use std::{any::Any, rc::Rc, sync::RwLock};

use crate::{devices::DeviceData, hart::privilege::PrivilegeMode};

use super::{address::Address, Memory};

// The type inside the box is big, but some indirection wouldnt
// really help
#[allow(clippy::type_complexity)]
pub enum Register {
    Const(u64),
    Poll {
        data: DeviceData,
        get: Box<dyn Fn(&Box<dyn Any + Send + Sync>) -> u64>,
        set: Box<dyn Fn(&mut Box<dyn Any + Send + Sync>, u64)>,
    },
}

// pub struct Register {
//     value: u64,
//     permission: PrivilegeMode,
// }

impl Register {
    pub fn get(&self) -> u64 {
        match self {
            Register::Const(v) => *v,
            Register::Poll { data, get, .. } => get(&data.read().unwrap()),
        }
    }

    pub fn set(&mut self, value: u64) {
        match self {
            Register::Const(v) => {}
            Register::Poll { data, set, .. } => set(&mut data.write().unwrap(), value),
        }
    }
}

// impl Register {
//     pub(super) fn new(permission: PrivilegeMode) -> Self {
//         Self {
//             value: 0,
//             permission,
//         }
//     }
//
//     pub fn set(&mut self, value: u64) {
//         self.value = value;
//     }
// }

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
