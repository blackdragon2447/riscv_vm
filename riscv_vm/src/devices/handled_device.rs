use std::{
    fmt::Debug,
    sync::mpsc::{self, Receiver, Sender},
};

use crate::memory::{registers::MemoryRegisterHandle, DeviceMemory};

use super::{
    event_bus::{self, DeviceEvent},
    DeviceError, DeviceInitError, DeviceObject,
};

pub trait HandledDevice: Debug + DeviceObject {
    fn update(&mut self, mem: &mut DeviceMemory) -> Result<(), DeviceError>;

    fn event(&mut self, mem: &mut DeviceMemory, event: DeviceEvent) -> Result<(), DeviceError>;
}

#[derive(Debug)]
pub struct HandledDeviceHolder {
    device: Box<dyn HandledDevice>,
    event_bus: Receiver<DeviceEvent>,
}

impl HandledDeviceHolder {
    pub fn new(device: Box<dyn HandledDevice>) -> (Sender<DeviceEvent>, Self) {
        let (s, r) = mpsc::channel();
        (
            s,
            Self {
                device,
                event_bus: r,
            },
        )
    }

    pub fn init_device(
        &mut self,
        mem: &mut DeviceMemory,
        registers: MemoryRegisterHandle<'_>,
    ) -> Result<(), DeviceInitError> {
        DeviceObject::init(self.device.as_mut(), mem, registers)
    }

    pub fn update(&mut self, mem: &mut DeviceMemory) -> Result<(), DeviceError> {
        self.device.update(mem)?;

        for e in self.event_bus.try_iter() {
            self.device.event(mem, e)?
        }

        Ok(())
    }
}
