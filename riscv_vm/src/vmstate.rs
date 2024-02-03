use std::{collections::HashMap, fmt::Debug};

use elf_load::{
    data::{Bitness, Endianess, ProgramType, ASI},
    ByteRanges, Elf,
};

use crate::{
    decode::decode,
    devices::{AsyncDevice, Device, DeviceError, DeviceInitError, HandledDevice},
    execute::{execute_rv64, ExecuteError},
    hart::{privilege::PrivilegeMode, Hart},
    memory::{address::Address, pmp::PMP, DeviceMemory, Memory, MemoryError},
};

#[derive(Default, Debug, Clone, Copy)]
pub struct VMSettings {
    pub pmp_enable: bool,
    pub virt_mem_enable: bool,
}

#[derive(Default, Debug)]
pub struct VMStateBuilder<const MEM_SIZE: usize> {
    hart_count: u64, //TODO: Change to vec HartSettings at some point
    settings: VMSettings,
}

pub struct VMState {
    harts: Vec<Hart>,
    mem: Memory,
    sync_devices: HashMap<usize, Box<dyn HandledDevice>>,
    // async_devices: HashMap<usize, Box<dyn AsyncDevice>>,
    next_dev_id: usize,
    settings: VMSettings,
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
    FetchError(MemoryError),
    InvalidElfKernel(KernelError),
    NoDeviceMemory,
    DeviceError(DeviceError),
    ExecureError(ExecuteError),
}

impl<const MEM_SIZE: usize> VMStateBuilder<MEM_SIZE> {
    pub fn enable_pmp(mut self) -> Self {
        self.settings.pmp_enable = true;
        self
    }

    pub fn enable_virt_mem(mut self) -> Self {
        self.settings.pmp_enable = true;
        self
    }

    pub fn set_hart_count(mut self, harts: u64) -> Self {
        self.hart_count = harts;
        self
    }

    pub fn build(self) -> VMState {
        VMState::new::<MEM_SIZE>(self.hart_count, self.settings)
    }
}

impl VMState {
    fn new<const MEM_SIZE: usize>(hart_count: u64, settings: VMSettings) -> Self {
        let mut harts = Vec::new();
        for i in 0..hart_count {
            harts.push(Hart::new(i, settings));
        }

        Self {
            harts,
            mem: Memory::new::<MEM_SIZE>(),
            sync_devices: HashMap::new(),
            // async_devices: HashMap::new(),
            next_dev_id: 0,
            settings,
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

    pub fn add_sync_device<D: Device + HandledDevice + 'static>(
        &mut self,
        // mem_size: u64,
        addr: Address,
    ) -> Result<(), DeviceInitError> {
        let mut memory = DeviceMemory::new(D::MEN_SIZE, addr);
        self.sync_devices
            .insert(self.next_dev_id, Box::new(D::init(&mut memory)?));
        self.mem.add_device_memory(self.next_dev_id, memory)?;
        self.next_dev_id += 1;
        Ok(())
    }

    pub fn add_async_device<D: Device + AsyncDevice + 'static>(
        &mut self,
        // mem_size: u64,
        addr: Address,
    ) -> Result<(), DeviceInitError> {
        let mut memory = DeviceMemory::new(D::MEN_SIZE, addr);
        // self.async_devices
        //     .insert(self.next_dev_id, Box::new(D::init(&mut memory)?));
        let mem = self.mem.add_device_memory(self.next_dev_id, memory)?;
        std::thread::spawn(move || -> Result<(), DeviceInitError> {
            let device = D::init(&mut mem.write().unwrap())?;
            device.run(mem);
            Ok(())
        });
        self.next_dev_id += 1;
        Ok(())
    }

    pub fn step(&mut self) -> Result<(), VMError> {
        for hart in &mut self.harts {
            hart.step(&mut self.mem)?;
        }
        // self.mem.update_devices();

        for dev in &mut self.sync_devices {
            dev.1.update(
                &mut *self
                    .mem
                    .get_device_memory(dev.0)?
                    .ok_or(VMError::NoDeviceMemory)?,
            )?;
        }

        Ok(())
    }

    pub fn step_hart_until(&mut self, hart: usize, target: Address) -> Result<(), VMError> {
        self.harts[hart].step_until(&mut self.mem, target)
    }

    #[cfg(test)]
    pub fn mem(&self) -> &Memory {
        &self.mem
    }

    pub fn dump_mem(&self) {
        self.mem.dump();
    }
}

impl Debug for VMState {
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

impl From<DeviceError> for VMError {
    fn from(value: DeviceError) -> Self {
        Self::DeviceError(value)
    }
}

impl From<ExecuteError> for VMError {
    fn from(value: ExecuteError) -> Self {
        Self::ExecureError(value)
    }
}

fn load_elf_phys(elf: &Elf, mem: &mut Memory) -> Result<Address, MemoryError> {
    for h in &elf.program_headers {
        if h.program_type == ProgramType::Load && h.seg_m_size.0 != 0 {
            let bytes = elf.bytes.get_bytes(h.seg_offset, h.seg_f_size.0);
            mem.write_bytes(bytes, h.seg_v_addr.into(), PrivilegeMode::Machine, None)?;
        }
    }

    Ok(elf.header.entry.into())
}
