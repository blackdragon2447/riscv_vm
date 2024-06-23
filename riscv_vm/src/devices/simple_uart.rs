use std::sync::{Arc, RwLock};

use crate::{
    hart::registers,
    memory::{
        memory_buffer::{MemoryBuffer, NaiveBuffer},
        registers::MemoryRegisterHandle,
        DeviceMemory,
    },
};

use super::{
    event_bus::DeviceEventBusHandle, handled_device::HandledDevice, Device, DeviceData,
    DeviceError, DeviceEvent, DeviceInitError, DeviceMemHandle, DeviceObject,
};

/// It's not uart and probably breaks if you look at it wrong.
#[derive(Debug)]
pub struct SimpleUart;

// struct UartMem([u8; 8]);

impl Device for SimpleUart {
    /// Hint for vm's using this device, a vm may give more/less memory.
    const MEM_SIZE: u64 = 8;

    fn new() -> Self {
        Self
    }
}

impl DeviceObject for SimpleUart {
    fn init(&mut self, mut mem: DeviceMemHandle) -> Result<(), DeviceInitError> {
        let mut dev_mem = NaiveBuffer::<8>::new();
        // dev_mem.0[5] |= 0x40;
        dev_mem.write_bytes(
            &[dev_mem.read_bytes(5u64.into(), 1).unwrap()[0] | 0x40],
            5u64.into(),
        );
        mem.add_memory_buffer(0x10000000u64.into(), dev_mem)?;
        Ok(())
    }
}

impl HandledDevice for SimpleUart {
    fn update(
        &mut self,
        mem: &mut DeviceMemory,
        _: &DeviceEventBusHandle,
        _: DeviceData,
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
