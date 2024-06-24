//!
//! # Devices
//!
//! Devices are the main way of extending vm behavior and adding interfaces.
//!
//! There devices come in two forms, handled devices and async devices, handled devices
//! are updated each clock cycle and are intended for simple short jobs while async devices
//! run independently of the main thread/clock and are intended for anything else, they have to
//! request their next update by returning a [`AsyncDeviceUpdateResult`][crate::devices::async_device::AsyncDeviceUpdateResult].
//!
//! ## Memory
//! Devices are permitted to add memory regions to the vm's memory, the behavour of this
//! memory is completely up to the device, with the only requirement being that these regions
//! implement [`MemoryBuffer`].

use std::{
    any::Any,
    collections::btree_map::Range,
    error::Error,
    rc::Rc,
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, PoisonError, RwLock,
    },
    time::{Duration, Instant},
};

pub use crate::memory::memory_buffer;
use crate::{
    memory::{memory_buffer::MemoryBuffer, Memory},
    Address,
};

pub mod async_device;
pub mod handled_device;
pub mod simple_uart;
#[cfg(feature = "vga_text_buf")]
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
    UpdateError(Box<dyn Error + Send>),
}

/// Gives devices access to memory during their initialization, allows for the registering of
/// device memory regions.
pub struct DeviceMemHandle<'a> {
    mem: &'a mut Memory,
}

impl<'a> DeviceMemHandle<'a> {
    pub(crate) fn new(mem: &'a mut Memory) -> Self {
        Self { mem }
    }

    /// Register a memory region to live at `base`, the buffer is consumed, but
    /// unless it could not be added, a refecence is given back, it is up to the device
    /// to store this refrence for later usage (read/writing data).
    pub fn add_memory_buffer<M>(
        &mut self,
        base: Address,
        buf: M,
    ) -> Result<Arc<RwLock<M>>, DeviceInitError>
    where
        M: MemoryBuffer + 'static,
    {
        self.mem.add_device_memory(base, buf)
    }
}

/// Part one of the trifecta of traits that make up a device, defines the size of memory shared
/// with the vm and the [`Device::new()`] function, which defines how to create (but not initialize) the
/// device, essentially the pre poweron state of the device.
pub trait Device {
    const MEM_SIZE: u64;

    fn new() -> Self;
}

/// Part two of the trifecta of traits that make up a device. The init functions is ran when
/// the vm and devices are initializing but before any code is ran, this is the time when the
/// device can register any memory regions, this initialization is allowed to error, these
/// errors should be passed up as [`DeviceInitError::Other`].
pub trait DeviceObject {
    fn init(&mut self, mem: DeviceMemHandle) -> Result<(), DeviceInitError>;
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
