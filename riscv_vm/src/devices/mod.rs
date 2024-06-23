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
//! ## Data Storage
//!
//! Devices are offered 3 forms of data storage. The first is the object that implements the
//! device traits, this form of data storage is internal to the device and not accessible to
//! the vm. Second there is [`DeviceMemory`], this is memory shared between the device and the vm,
//! the size of this memory is requested by setting the MEM_SIZE constant in the [`Device`] trait,
//! the location of this memory in the device's memory map is determined when instantating the
//! device, this memory may allow data races and non exclusive write access, devices may define
//! their own ways of communicating with the vm would they want to avoid this. The third form
//! of data is available as `DeviceData` returned when initializing a device, this data is given
//! back to the device on update and is also given to poll memory registers, not that this data is
//! given as a reference to a [`Box<dyn Any>`], the user is responsible for casting to the right
//! type.
//!

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

pub use crate::memory::DeviceMemory;
use crate::{
    memory::{memory_buffer::MemoryBuffer, Memory},
    Address,
};

pub mod async_device;
pub mod handled_device;
pub mod simple_uart;
#[cfg(feature = "vga_text_buf")]
pub mod vga_text_mode;

#[deprecated]
pub(crate) type DeviceId = usize;

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

pub struct DeviceMemHandle<'a> {
    mem: &'a mut Memory,
}

impl<'a> DeviceMemHandle<'a> {
    pub(crate) fn new(mem: &'a mut Memory) -> Self {
        Self { mem }
    }

    pub fn add_memory_buffer<M: MemoryBuffer + 'static>(
        &mut self,
        base: Address,
        buf: M,
    ) -> Result<Arc<RwLock<M>>, DeviceInitError> {
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

/// Alias for the wrapper around the simple data shared between devices and their memory mapped
/// registers, this type is given to the vm by the device upon init and is passed back to the
/// devices and registers on any event, the [`dyn Any + Send + Sync`] is guarenteed to be of the same
/// type as the data created by the device before being cast to an [`Any`].
///
/// NOTE: May at some point be converted to to a wrapper type, breaking current implementations.
#[deprecated]
pub type DeviceData = Arc<RwLock<Box<dyn Any + Send + Sync>>>;

/// Part two of the trifecta of traits that make up a device. The init functions is ran when
/// the vm and devices are initializing but before any code is ran, this is the time when the
/// device can register any memory mapped registers and return sync data to be shared with its
/// registers, this initialization is allowed to error, these errors should be passed up as
/// [`DeviceInitError::Other`].
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
