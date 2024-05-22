use std::{any::Any, rc::Rc, sync::RwLock};

use crate::{devices::DeviceData, hart::privilege::PrivilegeMode};

use super::{address::Address, Memory};

/// A memory mapped register, may be used by a devices to place constant or dynamic value in memory
/// outside its assigned [`crate::memory::DeviceMemory`] range, constants are set once and then unchangable,
/// dynamic (called poll) registers define functions for getting and setting them and are provided
/// with some data shared with the device.
pub struct Register {
    internal: RegisterType,
    pub length: RegisterLength,
}

impl Register {
    pub fn new_const(length: RegisterLength, value: u128) -> Self {
        Self {
            internal: RegisterType::Const(value),
            length,
        }
    }

    pub fn new_poll(
        length: RegisterLength,
        data: DeviceData,
        get: Box<dyn Fn(&Box<dyn Any + Send + Sync>) -> u128>,
        set: Box<dyn Fn(&mut Box<dyn Any + Send + Sync>, u128)>,
    ) -> Self {
        Self {
            internal: RegisterType::Poll { data, get, set },
            length,
        }
    }

    pub fn get(&self) -> u128 {
        match self.length {
            RegisterLength::U8 => self.internal.get() as u8 as u128,
            RegisterLength::U16 => self.internal.get() as u16 as u128,
            RegisterLength::U32 => self.internal.get() as u32 as u128,
            RegisterLength::U64 => self.internal.get() as u16 as u128,
            RegisterLength::U128 => self.internal.get(),
        }
    }

    pub fn set(&mut self, value: u128) {
        match self.length {
            RegisterLength::U8 => self.internal.set(value as u8 as u128),
            RegisterLength::U16 => self.internal.set(value as u16 as u128),
            RegisterLength::U32 => self.internal.set(value as u32 as u128),
            RegisterLength::U64 => self.internal.set(value as u64 as u128),
            RegisterLength::U128 => self.internal.set(value),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum RegisterLength {
    U8 = 1,
    U16 = 2,
    U32 = 4,
    U64 = 8,
    U128 = 16,
}

// The type inside the box is big, but some indirection wouldnt
// really help
#[allow(clippy::type_complexity)]
enum RegisterType {
    Const(u128),
    Poll {
        data: DeviceData,
        get: Box<dyn Fn(&Box<dyn Any + Send + Sync>) -> u128>,
        set: Box<dyn Fn(&mut Box<dyn Any + Send + Sync>, u128)>,
    },
}

// pub struct Register {
//     value: u64,
//     permission: PrivilegeMode,
// }

impl RegisterType {
    pub fn get(&self) -> u128 {
        match self {
            RegisterType::Const(v) => *v,
            RegisterType::Poll { data, get, .. } => get(&data.read().unwrap()),
        }
    }

    pub fn set(&mut self, value: u128) {
        match self {
            RegisterType::Const(v) => {}
            RegisterType::Poll { data, set, .. } => set(&mut data.write().unwrap(), value),
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
    pub(crate) fn new(memory: &'a mut Memory, dev_id: usize) -> MemoryRegisterHandle<'a> {
        Self {
            memory_ref: memory,
            dev_id,
        }
    }
}
