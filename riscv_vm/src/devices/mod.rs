use std::{collections::btree_map::Range, error::Error};

use elf_load::Address;

use crate::memory::DeviceMemory;

pub mod simple_uart;

#[derive(Debug)]
pub enum DeviceInitError {
    InsufficientMemory,
    MemoryOverlap,
}

#[derive(Debug)]
pub enum DeviceError {
    MemoryOverlap,
    UpdateError(Box<dyn Error>),
}

pub trait Device {
    fn init(mem: &mut DeviceMemory) -> Result<Self, DeviceInitError>
    where
        Self: Sized;
}

pub trait HandledDevice {
    fn update(&mut self, mem: &mut DeviceMemory) -> Result<(), DeviceError>;
}

impl<T: Error + 'static> From<T> for DeviceError {
    fn from(value: T) -> Self {
        Self::UpdateError(Box::new(value))
    }
}
