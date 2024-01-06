use core::fmt;
use std::{
    error::Error,
    fmt::Debug,
    ops::{Range, RangeBounds},
};

use elf_load::ByteRanges;

use super::{address::Address, Memory, MemoryError};

pub trait MemMapDevice {
    fn update(&mut self, memory: &mut [u8]) -> Result<(), DeviceError>;

    fn init_mem(&mut self, memory: &mut [u8]);
}

pub(super) struct MemMapDeviceState {
    device: Box<dyn MemMapDevice>,
    memory: Vec<u8>,
    range: Range<Address>,
}

impl Debug for MemMapDeviceState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MemMapDeviceState")
            .field("memory", &self.memory)
            .field("range", &self.range)
            .finish_non_exhaustive()
    }
}

#[derive(Debug)]
pub enum DeviceError {
    MemoryOverlap,
    UpdateError(Box<dyn Error>),
}

impl MemMapDeviceState {
    fn init(device: Box<dyn MemMapDevice>, memory: Vec<u8>, range: Range<Address>) -> Self {
        let mut device = Self {
            device,
            memory,
            range,
        };

        device.device.init_mem(&mut device.memory[..]);

        device
    }

    pub(super) fn in_range(&self, addr: Address) -> bool {
        self.range.contains(&addr)
    }

    pub(super) fn update(&mut self) {
        self.device.update(&mut self.memory);
    }

    pub fn write_bytes(&mut self, bytes: &[u8], addr: Address) -> Result<(), MemoryError> {
        if (self.range.contains(&addr)) {
            let idx = addr - self.range.start;
            if <Address as Into<usize>>::into(idx) > self.memory.len() {
                return Err(MemoryError::OutOfBoundsWrite);
            }
            if <Address as Into<usize>>::into(idx) + bytes.len() > self.memory.len() {
                return Err(MemoryError::OutOfMemory);
            }
            self.memory[idx.into()..(<Address as Into<usize>>::into(idx) + bytes.len())]
                .copy_from_slice(bytes);
        }
        Ok(())
    }

    pub fn read_bytes(&self, addr: Address, size: usize) -> Result<&[u8], MemoryError> {
        if (self.range.contains(&addr)) {
            let idx = addr - self.range.start;
            if <Address as Into<usize>>::into(idx) + size < self.memory.len() {
                Ok(self.memory.get_bytes(idx.into(), size as u64))
            } else {
                Err(MemoryError::OutOfBoundsRead(addr))
            }
        } else {
            Err(MemoryError::OutOfBoundsRead(addr))
        }
    }
}

impl<const SIZE: usize> Memory<SIZE> {
    pub fn add_mem_map_device<const DEV_SIZE: usize>(
        &mut self,
        device: Box<dyn MemMapDevice>,
        addr: Address,
    ) -> Result<(), DeviceError> {
        let range = addr..(addr + DEV_SIZE as u64);
        dbg!(&range);
        let memory = Vec::from([0u8; DEV_SIZE]);
        for dev in &self.devices {
            if dev.range.contains(&range.start)
                || dev.range.contains(&range.end)
                || range.contains(&dev.range.start)
                || range.contains(&dev.range.end)
            {
                return Err(DeviceError::MemoryOverlap);
            }
        }

        self.devices
            .push(MemMapDeviceState::init(device, memory, range));

        Ok(())
    }

    pub fn update_devices(&mut self) {
        for dev in &mut self.devices {
            dev.update();
            // dbg!(dev);
        }
    }
}

impl<T: Error + 'static> From<T> for DeviceError {
    fn from(value: T) -> Self {
        Self::UpdateError(Box::new(value))
    }
}
