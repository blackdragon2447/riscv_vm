use std::sync::mpsc::Sender;

use nohash_hasher::IntMap;

use super::VMSettings;
use crate::{
    devices::{
        async_device::{AsyncDevice, AsyncDeviceHolder},
        event_bus::DeviceEvent,
        handled_device::{HandledDevice, HandledDeviceHolder},
        Device, DeviceId, DeviceInitError,
    },
    memory::{address::Address, DeviceMemory},
    vmstate::VMState,
};

#[derive(Default, Debug)]
pub struct VMStateBuilder<const MEM_SIZE: usize> {
    hart_count: u64, //TODO: Change to vec HartSettings at some point
    settings: VMSettings,
    handled_devices: IntMap<DeviceId, (u64, Address, (Sender<DeviceEvent>, HandledDeviceHolder))>,
    async_devices: IntMap<DeviceId, (u64, Address, (Sender<DeviceEvent>, AsyncDeviceHolder))>,
    interrupt_controllder: Option<(
        DeviceId,
        (u64, Address, (Sender<DeviceEvent>, AsyncDeviceHolder)),
    )>,
    next_dev_id: usize,
}

#[derive(Debug)]
pub enum VMInitError {
    DeviceInitError(DeviceInitError),
}

impl<const MEM_SIZE: usize> VMStateBuilder<MEM_SIZE> {
    pub fn enable_pmp(mut self) -> Self {
        self.settings.pmp_enable = true;
        self
    }

    pub fn enable_virt_mem(mut self) -> Self {
        self.settings.pmp_enable = true;
        self
    }

    pub fn set_hart_count(mut self, harts: u64) -> Self {
        self.hart_count = harts;
        self
    }

    pub fn add_interrupt_controllder<D: Device + AsyncDevice + 'static>(
        mut self,
        addr: Address,
    ) -> Self {
        let device = Box::new(D::new());
        let dev = AsyncDeviceHolder::new(device);
        self.interrupt_controllder = Some((self.next_dev_id, (D::MEM_SIZE, addr, dev)));
        self.next_dev_id += 1;

        self
    }

    pub fn add_sync_device<D: Device + HandledDevice + 'static>(mut self, addr: Address) -> Self {
        let device = Box::new(D::new());
        let dev = HandledDeviceHolder::new(device);
        self.handled_devices
            .insert(self.next_dev_id, (D::MEM_SIZE, addr, dev));
        self.next_dev_id += 1;
        self
    }

    pub fn add_async_device<D: Device + AsyncDevice + 'static>(mut self, addr: Address) -> Self {
        let device = Box::new(D::new());
        let dev = AsyncDeviceHolder::new(device);
        self.async_devices
            .insert(self.next_dev_id, (D::MEM_SIZE, addr, dev));
        self.next_dev_id += 1;
        self
    }

    pub fn build(self) -> Result<VMState, VMInitError> {
        let mut state = VMState::new::<MEM_SIZE>(self.hart_count, self.settings);
        if let Some((k, v)) = self.interrupt_controllder {
            state.add_async_device(v.2, v.1, k, v.0, true)?;
        }
        for (k, v) in self.handled_devices {
            state.add_sync_device(v.2, v.1, k, v.0)?;
        }
        for (k, v) in self.async_devices {
            state.add_async_device(v.2, v.1, k, v.0, false)?;
        }
        Ok(state)
    }
}

impl From<DeviceInitError> for VMInitError {
    fn from(value: DeviceInitError) -> Self {
        Self::DeviceInitError(value)
    }
}
