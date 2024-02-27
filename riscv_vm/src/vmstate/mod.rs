pub mod builder;

use std::{
    collections::HashMap,
    fmt::Debug,
    sync::mpsc::{self, Receiver, Sender},
};

use elf_load::{
    data::{Bitness, Endianess, ProgramType, ASI},
    ByteRanges, Elf,
};

use crate::{
    decode::{decode, instruction::Instruction},
    devices::{
        async_device::{AsyncDevice, AsyncDeviceHolder},
        event_bus::{DeviceEvent, DeviceEventBus},
        handled_device::{HandledDevice, HandledDeviceHolder},
        Device, DeviceError, DeviceId, DeviceInitError,
    },
    execute::{execute_rv64, ExecuteError},
    hart::{self, privilege::PrivilegeMode, Hart},
    memory::{self, address::Address, pmp::PMP, DeviceMemory, Memory, MemoryError},
};

#[derive(Default, Debug, Clone, Copy)]
pub struct VMSettings {
    pub pmp_enable: bool,
    pub virt_mem_enable: bool,
}

pub struct VMState {
    harts: Vec<Hart>,
    mem: Memory,
    sync_devices: HashMap<usize, HandledDeviceHolder>,
    // async_devices: HashMap<usize, Box<dyn AsyncDevice>>,
    device_event_bus: DeviceEventBus,
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
    StepUntilLimit,
    DeviceError(DeviceError),
    ExecureError(ExecuteError),
    MBreak,
}

impl VMState {
    fn new<const MEM_SIZE: usize>(hart_count: u64, settings: VMSettings) -> Self {
        let mut harts = Vec::new();
        for i in 0..hart_count {
            harts.push(Hart::new(i, settings));
        }

        let (s, bus) = DeviceEventBus::new();

        Self {
            harts,
            mem: Memory::new::<MEM_SIZE>(s),
            sync_devices: HashMap::new(),
            // async_devices: HashMap::new(),
            device_event_bus: bus,
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

    pub fn add_sync_device(
        &mut self,
        mut dev: (Sender<DeviceEvent>, HandledDeviceHolder),
        addr: Address,
        id: DeviceId,
        mem_size: u64,
    ) -> Result<(), DeviceInitError> {
        let mut memory = DeviceMemory::new(mem_size, addr);
        dev.1
            .init_device(&mut memory, self.mem.register_handle(id))?;
        self.sync_devices.insert(id, dev.1);
        self.mem.add_device_memory(id, memory);
        self.device_event_bus.add_device(id, dev.0);
        Ok(())
    }

    pub fn add_async_device(
        &mut self,
        mut dev: (Sender<DeviceEvent>, AsyncDeviceHolder),
        addr: Address,
        id: DeviceId,
        mem_size: u64,
    ) -> Result<(), DeviceInitError> {
        let mut memory = DeviceMemory::new(mem_size, addr);
        dev.1
            .init_device(&mut memory, self.mem.register_handle(id))?;
        self.device_event_bus.add_device(id, dev.0);
        let mem = self.mem.add_device_memory(id, memory)?;
        dev.1.run(mem);
        Ok(())
    }

    // pub fn add_sync_device<D: Device + HandledDevice + 'static>(
    //     &mut self,
    //     // mem_size: u64,
    //     addr: Address,
    // ) -> Result<(), DeviceInitError> {
    //     let mut memory = DeviceMemory::new(D::MEN_SIZE, addr);
    //     let device = Box::new(D::init(
    //         &mut memory,
    //         self.mem.register_handle(self.next_dev_id),
    //     )?);
    //     let (s, holder) = HandledDeviceHolder::new(device);
    //     self.sync_devices.insert(self.next_dev_id, holder);
    //     self.mem.add_device_memory(self.next_dev_id, memory)?;
    //     self.device_event_bus.add_device(self.next_dev_id, s);
    //     self.next_dev_id += 1;
    //     Ok(())
    // }

    pub fn step(&mut self, verbose: bool) -> Result<(), VMError> {
        for hart in &mut self.harts {
            hart.step(&mut self.mem, verbose)?;
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
        self.harts[hart].step_until(&mut self.mem, target, 10000)
    }

    pub fn step_all_until(&mut self, target: Address) -> Result<(), VMError> {
        for _ in 0..10000 {
            for hart in &mut self.harts {
                if hart.get_pc() != target {
                    hart.step(&mut self.mem, false)?;
                }
            }

            for dev in &mut self.sync_devices {
                dev.1.update(
                    &mut *self
                        .mem
                        .get_device_memory(dev.0)?
                        .ok_or(VMError::NoDeviceMemory)?,
                )?;
            }
        }

        for hart in &self.harts {
            if hart.get_pc() != target {
                return Err(VMError::StepUntilLimit);
            }
        }

        Ok(())
    }

    pub fn run(&mut self) -> Result<(), VMError> {
        loop {
            self.step(false)?
        }
    }

    #[cfg(test)]
    pub fn mem(&self) -> &Memory {
        &self.mem
    }

    pub fn fetch(&mut self, hart: usize) -> Result<Instruction, MemoryError> {
        self.harts[hart].fetch(&mut self.mem)
    }

    pub fn dump_mem(&self) {
        self.mem.dump();
    }

    pub fn get_hart(&self, hart: usize) -> Option<&Hart> {
        self.harts.get(hart)
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
