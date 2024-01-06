use std::fmt::Debug;

use elf_load::{
    data::{Bitness, Endianess, ProgramType, ASI},
    ByteRanges, Elf,
};

use crate::{
    decode::decode,
    execute::execute,
    hart::Hart,
    memory::{address::Address, Memory, MemoryError},
};

pub struct VMState<const MEM_SIZE: usize> {
    harts: Vec<Hart>,
    mem: Memory<MEM_SIZE>,
}

#[derive(Debug)]
pub enum KernelError {
    InvalidBitness(Bitness),
    InvalidEndianness(Endianess),
    InvalidASI(ASI),
}

#[derive(Debug)]
pub enum VMError {
    MemoryError(MemoryError),
    InvalidElfKernel(KernelError),
}

impl<const MEM_SIZE: usize> VMState<MEM_SIZE> {
    pub fn new(hart_count: u64) -> Self {
        let mut harts = Vec::new();
        for i in 0..hart_count {
            harts.push(Hart::new(i));
        }

        Self {
            harts,
            mem: Memory::new(),
        }
    }

    pub fn load_elf_kernel(&mut self, elf: &Elf) -> Result<(), VMError> {
        if elf.header.arch != ASI::RISCV {
            return Err(VMError::InvalidElfKernel(KernelError::InvalidASI(
                elf.header.arch,
            )));
        }
        if elf.header.endianess != Endianess::Little {
            return Err(VMError::InvalidElfKernel(KernelError::InvalidEndianness(
                elf.header.endianess,
            )));
        }
        if elf.header.bitness != Bitness::B64 {
            return Err(VMError::InvalidElfKernel(KernelError::InvalidBitness(
                elf.header.bitness,
            )));
        }
        let addr = load_elf_phys(elf, &mut self.mem)?;
        Ok(())
    }

    pub fn step(&mut self) -> Result<(), VMError> {
        for hart in &mut self.harts {
            // Unwrap here is safe since u32 expects 4 bytes and we alyaws read 4 bytes (read_bytes
            // will return an Err if it cannot).
            let inst = decode(u32::from_le_bytes(
                self.mem.read_bytes(hart.get_pc(), 4)?.try_into().unwrap(),
            ));
            dbg!(inst);
            execute(hart, &mut self.mem, inst);
        }

        Ok(())
    }
}

impl<const MEM_SIZE: usize> Debug for VMState<MEM_SIZE> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut f = f.debug_struct("VMState");
        for h in &self.harts {
            f.field(format!("hart_{}", h.get_hart_id()).as_str(), h);
        }
        f.field("mem", &"-- ommitted --".to_string());
        f.finish_non_exhaustive()
    }
}

impl From<MemoryError> for VMError {
    fn from(value: MemoryError) -> Self {
        Self::MemoryError(value)
    }
}

fn load_elf_phys<const SIZE: usize>(
    elf: &Elf,
    mem: &mut Memory<SIZE>,
) -> Result<Address, MemoryError> {
    for h in &elf.program_headers {
        if h.program_type == ProgramType::Load && h.seg_m_size.0 != 0 {
            let bytes = elf.bytes.get_bytes(h.seg_offset, h.seg_f_size.0);
            mem.write_bytes(bytes, h.seg_p_addr.into())?;
        }
    }

    Ok(elf.header.entry.into())
}
