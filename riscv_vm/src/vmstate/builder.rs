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

/// Used to setup a VM, create a builder using [`Default::default()`] or [`VMStateBuilder::new()`], see the methods below for available
/// options, use [`VMStateBuilder::build()`] to turn into a usable VMState.
// The two IntMap which are arguably complex stay completely within this module, so having them be
// this bad is fine
#[allow(clippy::type_complexity)]
#[derive(Default, Debug)]
pub struct VMStateBuilder<const MEM_SIZE: usize> {
    hart_count: u64, //TODO: Change to vec HartSettings at some point
    settings: VMSettings,
    handled_devices: Vec<HandledDeviceHolder>,
    async_devices: IntMap<DeviceId, (u64, Address, (Sender<DeviceEvent>, AsyncDeviceHolder))>,
    next_dev_id: usize,
}

#[derive(Debug)]
pub enum VMInitError {
    DeviceInitError(DeviceInitError),
}

impl<const MEM_SIZE: usize> VMStateBuilder<MEM_SIZE> {
    /// Create a default instance of the builder with custom settings
    pub fn new(settings: VMSettings) -> Self {
        Self {
            settings,
            ..Default::default()
        }
    }

    /// Enable pysical memory protections, the actual protection settings are then to be
    /// set by the software running in the vm.
    pub fn enable_pmp(mut self) -> Self {
        self.settings.pmp_enable = true;
        self
    }

    /// Enable support for virual memory, allowes the guest to set up page tables.
    pub fn enable_virt_mem(mut self) -> Self {
        self.settings.pmp_enable = true;
        self
    }

    /// Set the number of harts this vm has.
    ///
    /// NOTE: Will at some point be replaced with hart specific settings.
    pub fn set_hart_count(mut self, harts: u64) -> Self {
        self.hart_count = harts;
        self
    }

    #[deprecated]
    /// DEPRECATED, Does nothing
    /// Interrupt Contoller will be built in, only a toggle will be available
    pub fn add_interrupt_controllder<D: Device + AsyncDevice + 'static>(
        mut self,
        addr: Address,
    ) -> Self {
        unimplemented!();

        self
    }

    /// Add a handled/sync devicem the actual device is specified via the generic, the address
    /// specifies where the devices memory will be placed in the vm's memory
    // The device will be passed this base address so it can place its memory mapped registers
    // relative to this address
    pub fn add_sync_device<D: Device + HandledDevice + 'static>(mut self, addr: Address) -> Self {
        let device = Box::new(D::new());
        let dev = HandledDeviceHolder::new(device);
        self.handled_devices.push(dev.1);
        self.next_dev_id += 1;
        self
    }

    /// Add an async devicem the actual device is specified via the generic, the address
    /// specifies where the devices memory will be placed in the vm's memory
    // The device will be passed this base address so it can place its memory mapped registers
    // relative to this address
    pub fn add_async_device<D: Device + AsyncDevice + 'static>(mut self, addr: Address) -> Self {
        let device = Box::new(D::new());
        let dev = AsyncDeviceHolder::new(device);
        self.async_devices
            .insert(self.next_dev_id, (D::MEM_SIZE, addr, dev));
        self.next_dev_id += 1;
        self
    }

    /// Build a vm from this builder, consumes the builder
    pub fn build(self) -> Result<VMState, VMInitError> {
        let mut state = VMState::new::<MEM_SIZE>(self.hart_count, self.settings);
        for d in self.handled_devices {
            state.add_sync_device(d)?;
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
