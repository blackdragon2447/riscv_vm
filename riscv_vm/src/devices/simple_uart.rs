use std::sync::{Arc, RwLock};

use crate::{
    hart::registers,
    memory::{registers::MemoryRegisterHandle, DeviceMemory},
};

use super::{
    event_bus::DeviceEventBusHandle, handled_device::HandledDevice, Device, DeviceData,
    DeviceError, DeviceEvent, DeviceInitError, DeviceObject,
};

/// It's not uart and probably breaks if you look at it wrong.
#[derive(Debug)]
pub struct SimpleUart;

impl Device for SimpleUart {
    /// Hint for vm's using this device, a vm may give more/less memory.
    const MEM_SIZE: u64 = 8;

    fn new() -> Self {
        Self
    }
}

impl DeviceObject for SimpleUart {
    fn init(
        &mut self,
        mem: &mut DeviceMemory,
        registers: MemoryRegisterHandle,
    ) -> Result<DeviceData, DeviceInitError> {
        if mem.size() < 8 {
            Err(DeviceInitError::InsufficientMemory)
        } else {
            mem.get_mem_mut()[5] |= 0x40;
            Ok(Arc::new(RwLock::new(Box::new(()))))
        }
    }
}

impl HandledDevice for SimpleUart {
    fn update(
        &mut self,
        mem: &mut DeviceMemory,
        _: &DeviceEventBusHandle,
    ) -> Result<(), DeviceError> {
        let reg = mem.get_mem()[0];
        if reg != 0 {
            print!("{}", std::str::from_utf8(&[reg])?);
            mem.get_mem_mut()[0] = 0;
            mem.get_mem_mut()[5] |= 0x40;
        }

        Ok(())
    }

    fn event(
        &mut self,
        mem: &mut DeviceMemory,
        event: DeviceEvent,
        _: &DeviceEventBusHandle,
    ) -> Result<(), DeviceError> {
        Ok(())
    }
}
