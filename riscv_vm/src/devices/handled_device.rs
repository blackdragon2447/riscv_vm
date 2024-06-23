use std::{
    fmt::Debug,
    sync::mpsc::{self, Receiver, Sender},
};

use crate::memory::{registers::MemoryRegisterHandle, DeviceMemory, Memory};

use super::{
    event_bus::{self, DeviceEvent, DeviceEventBusHandle},
    DeviceData, DeviceError, DeviceInitError, DeviceMemHandle, DeviceObject,
};

/// Part three of a handled device, this trait defines the behaviour of a handled device. These
/// devices run in sync with the main clock and are meant for simple, short, tasks.
pub trait HandledDevice: Debug + DeviceObject {
    /// The main way for the device to do logic, this function is called once per clock cycle.
    fn update(
        &mut self,
        mem: &mut DeviceMemory,
        event_bus: &DeviceEventBusHandle,
        data: DeviceData,
    ) -> Result<(), DeviceError>;

    /// Called when an event occurs which the device needs to be notified of, the device may choose
    /// to ignore this.
    #[deprecated]
    fn event(
        &mut self,
        mem: &mut DeviceMemory,
        event: DeviceEvent,
        event_bus: &DeviceEventBusHandle,
    ) -> Result<(), DeviceError>;
}

#[derive(Debug)]
pub(crate) struct HandledDeviceHolder {
    device: Box<dyn HandledDevice>,
    event_bus: Receiver<DeviceEvent>,
    data: Option<DeviceData>,
}

impl HandledDeviceHolder {
    pub(crate) fn new(device: Box<dyn HandledDevice>) -> (Sender<DeviceEvent>, Self) {
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

    pub(crate) fn init_device(&mut self, mem: &mut Memory) -> Result<(), DeviceInitError> {
        DeviceObject::init(self.device.as_mut(), DeviceMemHandle::new(mem))?;
        Ok(())
    }

    pub(crate) fn update(
        &mut self,
        mem: &mut DeviceMemory,
        event_bus: &DeviceEventBusHandle,
    ) -> Result<(), DeviceError> {
        let data = self.data.clone().expect("Device run before initializing");
        self.device.update(mem, event_bus, data)?;

        for e in self.event_bus.try_iter() {
            self.device.event(mem, e, event_bus)?
        }

        Ok(())
    }
}
