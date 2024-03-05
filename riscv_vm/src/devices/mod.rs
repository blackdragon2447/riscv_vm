use std::{
    collections::btree_map::Range,
    error::Error,
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, PoisonError, RwLock,
    },
    time::{Duration, Instant},
};

use crate::memory::{address::Address, registers::MemoryRegisterHandle, DeviceMemory};

use self::event_bus::DeviceEvent;

pub mod async_device;
pub mod event_bus;
pub mod handled_device;
pub mod simple_uart;
#[cfg(feature = "vga_text_buf")]
pub mod vga_text_mode;

pub type DeviceId = usize;

#[derive(Debug)]
pub enum DeviceInitError {
    InsufficientMemory,
    MemoryOverlap,
    MemoryPoison,
    Other(Box<dyn Error + Send>),
}

#[derive(Debug)]
pub enum DeviceError {
    MemoryOverlap,
    UpdateError(Box<dyn Error + Send>),
}

pub trait Device {
    const MEM_SIZE: u64;

    fn new() -> Self;
}

pub trait DeviceObject {
    fn init(
        &mut self,
        mem: &mut DeviceMemory,
        registers: MemoryRegisterHandle,
    ) -> Result<(), DeviceInitError>;
}

impl<T: Error + Send + 'static> From<T> for DeviceError {
    fn from(value: T) -> Self {
        Self::UpdateError(Box::new(value))
    }
}

impl<T> From<PoisonError<T>> for DeviceInitError {
    fn from(value: PoisonError<T>) -> Self {
        Self::MemoryPoison
    }
}
