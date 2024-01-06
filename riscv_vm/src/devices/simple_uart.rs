use crate::memory::mem_map_device::{DeviceError, MemMapDevice};

/// Note, not a full impl, just enough to send data out of a vm.
pub struct SimpleUart;

impl MemMapDevice for SimpleUart {
    fn update(&mut self, memory: &mut [u8]) -> Result<(), DeviceError> {
        // dbg!(memory[0]);
        let reg = memory[0];
        if reg != 0 {
            print!("{}", std::str::from_utf8(&[reg])?);
            memory[0] = 0;
            memory[5] |= 0x40;
        }

        Ok(())
    }

    fn init_mem(&mut self, memory: &mut [u8]) {
        memory[5] |= 0x40;
    }
}
