use crate::memory::DeviceMemory;

use super::{Device, DeviceError, DeviceInitError, HandledDevice};

pub struct SimpleUart;

impl Device for SimpleUart {
    fn init(mem: &mut DeviceMemory) -> Result<Self, DeviceInitError>
    where
        Self: Sized,
    {
        if mem.size() < 8 {
            return Err(DeviceInitError::InsufficientMemory);
        } else {
            mem.get_mem_mut()[5] |= 0x40;
            Ok(Self)
        }
    }
}

impl HandledDevice for SimpleUart {
    fn update(&mut self, mem: &mut DeviceMemory) -> Result<(), DeviceError> {
        let reg = mem.get_mem()[0];
        if reg != 0 {
            print!("{}", std::str::from_utf8(&[reg])?);
            mem.get_mem_mut()[0] = 0;
            mem.get_mem_mut()[5] |= 0x40;
        }

        Ok(())
    }
}
