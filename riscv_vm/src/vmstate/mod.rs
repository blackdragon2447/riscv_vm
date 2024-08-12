//! The vmstate is the main  way to interact with the vm, is is created via a [`VMStateBuilder`]
//! and can than be interacted with directly.

mod builder;

use core::panic;
use std::{
    fmt::Debug,
    sync::{Arc, RwLock},
};

use elf_load::{
    data::{Bitness, Endianess, ProgramType, ASI},
    ByteRanges, Elf,
};

use crate::{
    decode::Instruction,
    devices::{
        async_device::AsyncDeviceHolder, handled_device::HandledDeviceHolder, DeviceError,
        DeviceInitError,
    },
    execute::ExecuteError,
    hart::{privilege::PrivilegeMode, Hart},
    interrupt::{swi_controller::SwiController, timer::MTimer},
    memory::{address::Address, memory_buffer::NullPage, Memory, MemoryError},
};

pub use builder::{VMInitError, VMStateBuilder};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VMSettings {
    pub pmp_enable: bool,
    pub virt_mem_enable: bool,

    pub timer_addr: Address,

    pub m_mode_swi_enable: bool,
    pub m_mode_swi_addr: Address,

    pub s_mode_swi_enable: bool,
    pub s_mode_swi_addr: Address,

    pub imsic_enable: bool,
    pub imsic_base: Address,

    pub reset_vec: Address,
}

impl Default for VMSettings {
    fn default() -> Self {
        Self {
            pmp_enable: true,
            virt_mem_enable: false,

            timer_addr: 0x1000.into(),

            m_mode_swi_enable: false,
            m_mode_swi_addr: 0x2000.into(),

            s_mode_swi_enable: false,
            s_mode_swi_addr: 0x3000.into(),

            imsic_enable: false,
            imsic_base: 0x100000.into(),

            reset_vec: 0x80000000u64.into(),
        }
    }
}

/// An actual instance of a riscv vm, with memory, devices and harts
#[allow(unused)]
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
    fn new(hart_count: u64, settings: VMSettings, mem_size: usize) -> Self {
        let mut mem = Memory::new(mem_size);

        let mut timer = MTimer::new(hart_count as usize);

        let mut harts = Vec::new();
        for i in 0..hart_count {
            let hart = Hart::new(i, settings, timer.get_ref());
            timer.add_interrupt_bits(i as usize, hart.get_mip_ref());
            harts.push(hart);
        }

        let timer = mem.add_device_memory(settings.timer_addr, timer).unwrap();

        if settings.s_mode_swi_enable {
            let s_swi = SwiController::new(&harts, PrivilegeMode::Supervisor);
            mem.add_device_memory(settings.s_mode_swi_addr, s_swi)
                .unwrap();
        }

        if settings.m_mode_swi_enable {
            let m_swi = SwiController::new(&harts, PrivilegeMode::Machine);
            mem.add_device_memory(settings.m_mode_swi_addr, m_swi)
                .unwrap();
        }

        if settings.imsic_enable {
            let c = 12;
            let k = (hart_count).ilog2() + 1;
            let boundry = 2u64.pow(k + c);
            if settings.imsic_base % boundry != 0 {
                panic!(
                    "Imsic base address does not align to a {:#X} address boundry, see --help for more",
                    boundry
                );
            }
            let m_base = settings.imsic_base;
            let s_base = settings.imsic_base + boundry;
            let align = 2u64.pow(c);
            for h in &mut harts {
                h.add_imsic(&mut mem, (m_base.into(), s_base.into()), align);
            }

            let mut null_page_base = m_base + (hart_count * align);
            while null_page_base % boundry != 0 {
                mem.add_device_memory(null_page_base, NullPage).unwrap();
                null_page_base += align;
            }

            let mut null_page_base = s_base + (hart_count * align);
            while null_page_base % boundry != 0 {
                mem.add_device_memory(null_page_base, NullPage).unwrap();
                null_page_base += align;
            }
        }

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
        let _addr = load_elf_phys(elf, &mut self.mem)?;
        Ok(())
    }

    fn add_sync_device(&mut self, mut dev: HandledDeviceHolder) -> Result<(), DeviceInitError> {
        dev.init_device(&mut self.mem)?;
        self.sync_devices.push(dev);
        Ok(())
    }

    fn add_async_device(&mut self, mut dev: AsyncDeviceHolder) -> Result<(), DeviceInitError> {
        dev.init_and_run_device(&mut self.mem);
        Ok(())
        // let mut memory = DeviceMemory::new(mem_size, addr);
        //
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
        for dev in &mut self.sync_devices {
            dev.update()?;
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
    pub fn fetch(&mut self, hart: usize) -> Result<(Instruction, bool), MemoryError> {
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
