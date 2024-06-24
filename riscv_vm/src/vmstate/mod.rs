//! The vmstate is the main  way to interact with the vm, is is created via a [`VMStateBuilder`]
//! and can than be interacted with directly.

mod builder;
pub(crate) mod timer;

use std::{
    any::Any,
    collections::HashMap,
    fmt::Debug,
    rc::Rc,
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, RwLock,
    },
};

use elf_load::{
    data::{Bitness, Endianess, ProgramType, ASI},
    ByteRanges, Elf,
};

use crate::{
    decode::{decode, Instruction},
    devices::{
        async_device::{AsyncDevice, AsyncDeviceHolder},
        handled_device::{HandledDevice, HandledDeviceHolder},
        Device, DeviceError, DeviceInitError,
    },
    execute::{execute_rv64, ExecuteError},
    hart::{self, privilege::PrivilegeMode, trap::InterruptTarget, Hart},
    memory::{self, address::Address, pmp::PMP, Memory, MemoryError},
};

use self::timer::{MTimer, TimerRef};
pub use builder::{VMInitError, VMStateBuilder};

#[derive(Default, Debug, Clone, Copy)]
pub struct VMSettings {
    pub pmp_enable: bool,
    pub virt_mem_enable: bool,
}

/// An actual instance of a riscv vm, with memory, devices and harts
pub struct VMState {
    harts: Vec<Hart>,
    mem: Memory,
    sync_devices: Vec<HandledDeviceHolder>,
    // async_devices: HashMap<usize, Box<dyn AsyncDevice>>,
    timer: Arc<RwLock<MTimer>>,
    next_dev_id: usize,
    settings: VMSettings,
}

#[derive(Debug)]
pub enum KernelLoadError {
    InvalidBitness(Bitness),
    InvalidEndianness(Endianess),
    InvalidASI(ASI),
}

#[derive(Debug)]
pub enum VMError {
    MemoryError(MemoryError),
    FetchError(MemoryError),
    InvalidElfKernel(KernelLoadError),
    NoDeviceMemory,
    StepUntilLimit,
    DeviceError(DeviceError),
    ExecureError(ExecuteError),
    MBreak,
}

impl VMState {
    fn new<const MEM_SIZE: usize>(hart_count: u64, settings: VMSettings) -> Self {
        let mut mem = Memory::new::<MEM_SIZE>();
        // let timer = MTimer::new(
        //     hart_count as usize,
        //     bus.get_handle(InterruptPermission::InterruptController),
        // );
        // let timer: DeviceData = Arc::new(RwLock::new(Box::new(timer)));
        // mem.add_timer(0x1000.into(), 0x1040.into(), timer.clone());

        let mut timer = MTimer::new(hart_count as usize);

        let mut harts = Vec::new();
        for i in 0..hart_count {
            let hart = Hart::new(i, settings, timer.get_ref());
            timer.add_interrupt_bits(i as usize, hart.get_mip_ref());
            harts.push(hart);
        }

        let timer = mem.add_device_memory(0x1000.into(), timer).unwrap();

        Self {
            harts,
            mem,
            sync_devices: Vec::new(),
            // async_devices: HashMap::new(),
            timer,
            next_dev_id: 0,
            settings,
        }
    }

    /// Load a kernel from an elf file and place it at 0x80000000 (bottom of memory)
    /// The elf must be riscv64 little endian.
    pub fn load_elf_kernel(&mut self, elf: &Elf) -> Result<(), VMError> {
        if elf.header.arch != ASI::RISCV {
            return Err(VMError::InvalidElfKernel(KernelLoadError::InvalidASI(
                elf.header.arch,
            )));
        }
        if elf.header.endianess != Endianess::Little {
            return Err(VMError::InvalidElfKernel(
                KernelLoadError::InvalidEndianness(elf.header.endianess),
            ));
        }
        if elf.header.bitness != Bitness::B64 {
            return Err(VMError::InvalidElfKernel(KernelLoadError::InvalidBitness(
                elf.header.bitness,
            )));
        }
        let addr = load_elf_phys(elf, &mut self.mem)?;
        Ok(())
    }

    fn add_sync_device(&mut self, mut dev: HandledDeviceHolder) -> Result<(), DeviceInitError> {
        dev.init_device(&mut self.mem);
        self.sync_devices.push(dev);
        Ok(())
        // let mut memory = DeviceMemory::new(mem_size, addr);
        // dev.1
        //     .init_device(&mut memory, self.mem.register_handle(id))?;
        // self.sync_devices.insert(id, dev.1);
        // self.mem.add_device_memory(id, memory);
        // self.device_event_bus.add_device(id, dev.0);
        // Ok(())
    }

    fn add_async_device(&mut self, mut dev: AsyncDeviceHolder) -> Result<(), DeviceInitError> {
        todo!()
        // let mut memory = DeviceMemory::new(mem_size, addr);
        // dev.1
        //     .init_device(&mut memory, self.mem.register_handle(id))?;
        // self.device_event_bus.add_device(id, dev.0);
        // let mem = self.mem.add_device_memory(id, memory)?;
        // dev.1.run(
        //     mem,
        //     self.device_event_bus.get_handle(if is_interupt_controller {
        //         InterruptPermission::InterruptController
        //     } else {
        //         InterruptPermission::Normal
        //     }),
        // );
        // Ok(())
    }

    /// Advance all cores one cycle and, if verbose, print the instruction that was executed
    pub fn step(&mut self, verbose: bool) -> Result<(), VMError> {
        // TODO
        // for dev in &mut self.sync_devices {
        //     dev.1.update(
        //         &mut *self
        //             .mem
        //             .get_device_memory(dev.0)?
        //             .ok_or(VMError::NoDeviceMemory)?,
        //         &self
        //             .device_event_bus
        //             .get_handle(InterruptPermission::Normal),
        //     )?;
        // }

        for dev in &mut self.sync_devices {
            dev.update().unwrap();
        }

        self.timer.read().unwrap().generate_interrupts();

        for hart in &mut self.harts {
            hart.step(&mut self.mem, verbose)?;
        }

        Ok(())
    }

    /// Step a specific hart until its pc hits the given address or it has made 10000 steps,
    /// whichever happens first
    pub fn step_hart_until(&mut self, hart: usize, target: Address) -> Result<(), VMError> {
        self.harts[hart].step_until(&mut self.mem, target, 10000)
    }

    /// Step all harts until its pc hits the given address or it has made 10000 steps,
    /// whichever happens first
    pub fn step_all_until(&mut self, target: Address) -> Result<(), VMError> {
        for _ in 0..10000 {
            for dev in &mut self.sync_devices {
                dev.update().unwrap();
            }

            for hart in &mut self.harts {
                if hart.get_pc() != target {
                    hart.step(&mut self.mem, false)?;
                }
            }

            // self.device_event_bus.distribute();

            // for i in self.device_event_bus.interrupts() {
            //     match i {
            //         crate::devices::event_bus::InterruptSignal::Set(t, i) => match t {
            //             InterruptTarget::All => {
            //                 for h in &mut self.harts {
            //                     h.interrupt(i);
            //                 }
            //             }
            //             InterruptTarget::Single(h) => {
            //                 if let Some(h) = self.harts.get_mut(h) {
            //                     h.interrupt(i);
            //                 }
            //             }
            //         },
            //         crate::devices::event_bus::InterruptSignal::Clear(t, i) => match t {
            //             InterruptTarget::All => {
            //                 for h in &mut self.harts {
            //                     h.clear_interrupt(i);
            //                 }
            //             }
            //             InterruptTarget::Single(h) => {
            //                 if let Some(h) = self.harts.get_mut(h) {
            //                     h.clear_interrupt(i);
            //                 }
            //             }
            //         },
            //     }
            // }
        }

        for hart in &self.harts {
            if hart.get_pc() != target {
                return Err(VMError::StepUntilLimit);
            }
        }

        Ok(())
    }

    /// Run the vm until it errors or forever, whichever happens first.
    pub fn run(&mut self) -> Result<(), VMError> {
        loop {
            self.step(false)?
        }
    }

    #[cfg(test)]
    pub(crate) fn mem(&self) -> &Memory {
        &self.mem
    }

    /// Attempt to fetch on a specific hart and return the decoded instruction
    pub fn fetch(&mut self, hart: usize) -> Result<Instruction, MemoryError> {
        self.harts[hart].fetch(&mut self.mem)
    }

    #[deprecated]
    pub fn dump_mem(&self) {
        self.mem.dump();
    }

    #[deprecated]
    pub fn print_mem_map(&self) {
        println!("{:#?}", self.mem.get_map());
    }

    /// Get an immutable refierence to a specific hart, if it exists.
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
            mem.write_bytes(bytes, h.seg_v_addr.into())?;
        }
    }

    Ok(elf.header.entry.into())
}
