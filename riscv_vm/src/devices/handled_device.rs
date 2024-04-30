use std::{
    fmt::Debug,
    sync::mpsc::{self, Receiver, Sender},
};

use crate::memory::{registers::MemoryRegisterHandle, DeviceMemory};

use super::{
    event_bus::{self, DeviceEvent, DeviceEventBusHandle},
    DeviceData, DeviceError, DeviceInitError, DeviceObject,
};

pub trait HandledDevice: Debug + DeviceObject {
    fn update(
        &mut self,
        mem: &mut DeviceMemory,
        event_bus: &DeviceEventBusHandle,
    ) -> Result<(), DeviceError>;

    fn event(
        &mut self,
        mem: &mut DeviceMemory,
        event: DeviceEvent,
        event_bus: &DeviceEventBusHandle,
    ) -> Result<(), DeviceError>;
}

#[derive(Debug)]
pub struct HandledDeviceHolder {
    device: Box<dyn HandledDevice>,
    event_bus: Receiver<DeviceEvent>,
    data: Option<DeviceData>,
}

impl HandledDeviceHolder {
    pub fn new(device: Box<dyn HandledDevice>) -> (Sender<DeviceEvent>, Self) {
        let (s, r) = mpsc::channel();
        (
            s,
            Self {
                device,
                event_bus: r,
                data: None,
            },
        )
    }

    pub fn init_device(
        &mut self,
        mem: &mut DeviceMemory,
        registers: MemoryRegisterHandle<'_>,
    ) -> Result<(), DeviceInitError> {
        let data = DeviceObject::init(self.device.as_mut(), mem, registers)?;
        self.data = Some(data);
        Ok(())
    }

    pub fn update(
        &mut self,
        mem: &mut DeviceMemory,
        event_bus: &DeviceEventBusHandle,
    ) -> Result<(), DeviceError> {
        self.device.update(mem, event_bus)?;

        for e in self.event_bus.try_iter() {
            self.device.event(mem, e, event_bus)?
        }

        Ok(())
    }
}
