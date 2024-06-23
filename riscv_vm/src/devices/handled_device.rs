use std::{
    fmt::Debug,
    sync::mpsc::{self, Receiver, Sender},
};

use crate::memory::{DeviceMemory, Memory};

use super::{DeviceError, DeviceInitError, DeviceMemHandle, DeviceObject};

/// Part three of a handled device, this trait defines the behaviour of a handled device. These
/// devices run in sync with the main clock and are meant for simple, short, tasks.
pub trait HandledDevice: Debug + DeviceObject {
    /// The main way for the device to do logic, this function is called once per clock cycle.
    fn update(&mut self) -> Result<(), DeviceError>;
}

#[derive(Debug)]
pub(crate) struct HandledDeviceHolder {
    device: Box<dyn HandledDevice>,
}

impl HandledDeviceHolder {
    pub(crate) fn new(device: Box<dyn HandledDevice>) -> (Sender<()>, Self) {
        let (s, r) = mpsc::channel();
        (s, Self { device })
    }

    pub(crate) fn init_device(&mut self, mem: &mut Memory) -> Result<(), DeviceInitError> {
        DeviceObject::init(self.device.as_mut(), DeviceMemHandle::new(mem))?;
        Ok(())
    }

    pub(crate) fn update(&mut self) -> Result<(), DeviceError> {
        self.device.update()
    }
}
