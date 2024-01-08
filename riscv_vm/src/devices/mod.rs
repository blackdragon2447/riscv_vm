use std::{
    collections::btree_map::Range,
    error::Error,
    sync::{Arc, PoisonError, RwLock},
};

use elf_load::Address;
use wgpu::CreateSurfaceError;
use winit::error::OsError;

use crate::memory::DeviceMemory;

pub mod simple_uart;
pub mod vga_text_mode;

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
    UpdateError(Box<dyn Error>),
}

pub trait Device {
    const MEN_SIZE: u64;

    fn init(mem: &mut DeviceMemory) -> Result<Self, DeviceInitError>
    where
        Self: Sized;
}

pub trait HandledDevice {
    fn update(&mut self, mem: &mut DeviceMemory) -> Result<(), DeviceError>;
}

pub trait AsyncDevice {
    fn run(self, mem: Arc<RwLock<DeviceMemory>>);
}

impl<T: Error + 'static> From<T> for DeviceError {
    fn from(value: T) -> Self {
        Self::UpdateError(Box::new(value))
    }
}

impl<T> From<PoisonError<T>> for DeviceInitError {
    fn from(value: PoisonError<T>) -> Self {
        Self::MemoryPoison
    }
}

impl From<OsError> for DeviceInitError {
    fn from(value: OsError) -> Self {
        Self::Other(Box::new(value))
    }
}

impl From<CreateSurfaceError> for DeviceInitError {
    fn from(value: CreateSurfaceError) -> Self {
        Self::Other(Box::new(value))
    }
}
