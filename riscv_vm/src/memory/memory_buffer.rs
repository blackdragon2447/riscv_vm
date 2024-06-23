use crate::Address;

use super::MemoryError;

#[derive(Debug)]
pub enum MemoryBufferError {
    OutOfBoundsWrite(Address),
    OutOfBoundsRead(Address),
    OutOfMemory,
}

/// All addr arguments are normalised so that 0x0 is at the base of the buffer
pub trait MemoryBuffer {
    fn size(&self) -> u64;

    fn write_bytes(&mut self, bytes: &[u8], addr: Address) -> Result<(), MemoryBufferError>;

    fn read_bytes(&self, addr: Address, size: usize) -> Result<Vec<u8>, MemoryBufferError>;
}

#[derive(Debug)]
pub(crate) struct NaiveBuffer<const SIZE: usize>([u8; SIZE]);

impl<const SIZE: usize> NaiveBuffer<SIZE> {
    pub fn new() -> Self {
        Self([0; SIZE])
    }
}

impl<const SIZE: usize> MemoryBuffer for NaiveBuffer<SIZE> {
    fn size(&self) -> u64 {
        SIZE as u64
    }

    fn write_bytes(
        &mut self,
        bytes: &[u8],
        addr: crate::Address,
    ) -> Result<(), crate::memory::memory_buffer::MemoryBufferError> {
        &mut self.0[(addr.into())..(<crate::Address as Into<usize>>::into(addr) + bytes.len())]
            .copy_from_slice(bytes);
        Ok(())
    }

    fn read_bytes(
        &self,
        addr: crate::Address,
        size: usize,
    ) -> Result<Vec<u8>, crate::memory::memory_buffer::MemoryBufferError> {
        Ok(self.0[(addr.into()..(<crate::Address as Into<usize>>::into(addr) + size))].to_vec())
    }
}

impl From<MemoryBufferError> for MemoryError {
    fn from(value: MemoryBufferError) -> Self {
        match value {
            MemoryBufferError::OutOfBoundsWrite(a) => MemoryError::OutOfBoundsWrite(a),
            MemoryBufferError::OutOfBoundsRead(a) => MemoryError::OutOfBoundsRead(a),
            MemoryBufferError::OutOfMemory => MemoryError::OutOfMemory,
        }
    }
}
