use crate::Address;

use super::MemoryError;

pub enum MemoryBufferError {
    OutOfBoundsWrite(Address),
    OutOfBoundsRead(Address),
    OutOfMemory,
}

/// All addr arguments are normalised so that 0x0 is at the base of the buffer
pub trait MemoryBuffer {
    fn write_bytes(&mut self, bytes: &[u8], addr: Address) -> Result<(), MemoryBufferError>;

    fn read_bytes(&self, addr: Address, size: usize) -> Result<Vec<u8>, MemoryBufferError>;
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
