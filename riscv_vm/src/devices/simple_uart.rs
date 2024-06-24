use core::panic;
use std::{
    io::{stdout, Write},
    sync::{Arc, RwLock},
};

use crate::{
    hart::registers,
    memory::memory_buffer::{MemoryBuffer, NaiveBuffer},
};

use super::{
    handled_device::HandledDevice, Device, DeviceError, DeviceInitError, DeviceMemHandle,
    DeviceObject,
};

/// It's not uart and probably breaks if you look at it wrong.
#[derive(Debug)]
pub struct SimpleUart(Option<Arc<RwLock<NaiveBuffer<8>>>>);

// struct UartMem([u8; 8]);

impl Device for SimpleUart {
    /// Hint for vm's using this device, a vm may give more/less memory.
    const MEM_SIZE: u64 = 8;

    fn new() -> Self {
        Self(None)
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
        self.0 = Some(mem.add_memory_buffer(0x10000000u64.into(), dev_mem)?);
        Ok(())
    }
}

impl HandledDevice for SimpleUart {
    fn update(&mut self) -> Result<(), DeviceError> {
        let mut mem = self.0.as_ref().unwrap().write().unwrap();
        let reg = mem.read_bytes(0u64.into(), 1).unwrap()[0];
        if reg != 0 {
            print!("{}", std::str::from_utf8(&[reg])?);
            stdout().flush();
            mem.write_bytes(&[0], 0u64.into());
            let byte = mem.read_bytes(5u64.into(), 1).unwrap()[0] | 0x40;
            mem.write_bytes(&[byte], 5u64.into()).unwrap();
        }

        Ok(())
    }
}
