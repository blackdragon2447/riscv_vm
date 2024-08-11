use enumflags2::{bitflags, BitFlag, BitFlags};
#[cfg(feature = "float")]
use softfloat_wrapper::ExceptionFlags;

use crate::{
    execute::ExecuteError,
    hart::{counters::Counters, isa::Isa, privilege::PrivilegeMode, CsrAddress},
    interrupt::timer::TimerRef,
    memory::{address::Address, paging::Satp, pmp::PMP},
    trap::{Exception, InterruptInternal},
};

#[cfg(feature = "float")]
use crate::decode::instruction::RoundingMode;

use std::{fmt::Debug, rc::Rc, sync::Mutex};

use super::{csr_address::CsrType, csrind::MiReg, CsrProvider};

#[repr(u8)]
#[derive(Debug, PartialEq, Eq)]
pub enum TrapMode {
    Direct = 0,
    Vectored = 1,
}

#[derive(Debug)]
pub struct TrapVector {
    pub(crate) mode: TrapMode,
    pub(crate) base: Address,
}

#[repr(u64)]
#[derive(Debug, Clone, Copy)]
#[bitflags]
#[cfg(feature = "float")]
pub enum FFlags {
    InvalidOperation = 1 << 4,
    DevideByZero = 1 << 3,
    Overflow = 1 << 2,
    Underflow = 1 << 1,
    Inexact = 1 << 0,
}

const S_INTERRUPT_MASK: u64 = 0b0000_0010_0010_0010;
const TOGGLEABLE_INTERRUPTS: u64 = 0b0000_1010_0010_0000;

pub struct CsrHolder {
    // UserMode
    #[cfg(feature = "float")]
    fflags: BitFlags<FFlags>,
    #[cfg(feature = "float")]
    frm: RoundingMode,
    // cycle (tied to mcylce)
    timer: TimerRef,
    // instret (tied to minstret)

    // SupervisorMode
    // sstatus (tied to status)
    pub(in crate::hart) sie: BitFlags<InterruptInternal>,
    pub(in crate::hart) stvec: TrapVector,
    scounteren: BitFlags<Counters>,
    senvcfg: u64,
    sscratch: u64,
    pub(in crate::hart) sepc: Address,
    pub(in crate::hart) scause: u64,
    pub(in crate::hart) stval: u64,
    siselect: u64,

    pub(in crate::hart) satp: Satp,

    // MachineMode
    /// RO 0
    mvendorid: u64,
    /// RO 0
    marchid: u64,
    /// RO 0
    mimpid: u64,
    mhartid: u64,
    mconfigptr: u64,
    misa: BitFlags<Isa>,
    pub(in crate::hart) medeleg: BitFlags<Exception>,
    pub(in crate::hart) mideleg: BitFlags<InterruptInternal>,
    pub(in crate::hart) mie: BitFlags<InterruptInternal>,
    pub(in crate::hart) mtvec: TrapVector,
    mcounteren: BitFlags<Counters>,
    mscratch: u64,
    pub(in crate::hart) mepc: Address,
    pub(in crate::hart) mcause: u64,
    pub(in crate::hart) mtval: u64,
    pub(in crate::hart) mip: Rc<Mutex<BitFlags<InterruptInternal>>>,
    menvcfg: u64,
    mseccfg: u64,

    miselect: u64,

    mcycle: u64,
    minstret: u64,
    mcounterinhibit: BitFlags<Counters>,

    pub pmp: PMP,

    csr_providers: Vec<Box<dyn CsrProvider>>,

    //Other
    pub(in crate::hart) status: Status,
}

/// State values for the FS VX XS Fields of mstatus, names of variants are for FS and VS, XS
/// meanings are in docs.
#[allow(clippy::upper_case_acronyms)]
#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FloatVectorXternalStatus {
    /// XS meaning: All off
    Off = 0,
    /// XS meaning: None dirty or clean, some on
    Initial = 1,
    /// XS meaning: None dirty, some clean
    Clean = 2,
    /// XS meaning: Some dirty
    Dirty = 3,
}

#[derive(Debug)]
pub struct Status {
    /// [1] M + S, Enable interrupts in supervisor mode while in that mode
    pub(crate) sie: bool, // always false (interrupts not impl)

    /// [3] M, Enable interrupts in machine mode while in that mode
    pub(crate) mie: bool, // always false (interrupts not impl)

    /// [5] M + S, Value of sie prior to taking a trap to S mode. Is set to
    /// the value of sie when a trap is taken and sie is set to this value on an sret.
    pub(crate) spie: bool,

    /// [7] M, Value of mie prior to taking a trap to M mode. Is set to the
    /// value of mie when a trap is taken and mie is set to this value on an mret.
    pub(crate) mpie: bool,

    /// [8] M + S, The privilege mode prior to taking a trap to S mode. Sret will set the
    /// privilege level to the value of this register, may never contain M or H
    pub(crate) spp: PrivilegeMode,

    /// [9..10] M + S, See privileged spec table 3.4 for exact state.
    pub(crate) vs: FloatVectorXternalStatus,

    /// [11..12] M, The privilege mode prior to taking a trap to M mode. Mret will
    /// set the privilege level to the value of this register.
    pub(crate) mpp: PrivilegeMode,

    /// [13..14] M + S, See privileged spec table 3.4 for exact state.
    pub(crate) fs: FloatVectorXternalStatus,

    /// [15..16] M + S, See privileged spec table 3.4 for exact state.
    pub(crate) xs: FloatVectorXternalStatus,

    /// [17] M, If set, memory accesses while in M mode are executed as if the privilege
    /// mode in mpp was the current privilege mode.
    pub(crate) mprv: bool,

    /// [18] M + S, If set any pages marked as user are also readable from supervisor mode.
    pub(crate) sum: bool,

    /// [19] M + S, If set any pages marked as executable but not readable are readable.
    pub(crate) mxr: bool,

    /// [20] M, If set, writes to satp, and SFENCE.VMA and SINVAL.VMA instructions will be trapped
    /// and throw an lllegal instruction.
    pub(crate) tvm: bool,

    /// [21] M, If set, wfi in S or U mode will cause an IllegalInstruction if it does not complete
    /// in a set time (0)
    pub(crate) tw: bool,

    /// [22] M, If set, sret in S mode will cause an IllegalInstruction
    pub(crate) tsr: bool,
}

impl Debug for CsrHolder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut dbg = f.debug_struct("CsrHolder");

        #[cfg(feature = "float")]
        dbg.field("fflags", &self.fflags).field("frm", &self.frm);
        dbg.field("stvec", &self.stvec)
            .field("scounteren", &self.scounteren)
            .field("senvcfg", &self.senvcfg)
            .field("sscratch", &self.sscratch)
            .field("sepc", &self.sepc)
            .field("scause", &self.scause)
            .field("stval", &self.stval)
            .field("satp", &self.satp)
            .field("mvendorid", &self.mvendorid)
            .field("marchid", &self.marchid)
            .field("mimpid", &self.mimpid)
            .field("mhartid", &self.mhartid)
            .field("mconfigptr", &self.mconfigptr)
            .field("misa", &self.misa)
            .field("medeleg", &self.medeleg)
            .field("mideleg", &self.mideleg)
            .field("mie", &self.mie)
            .field("mip", &self.mip)
            .field("mtvec", &self.mtvec)
            .field("mcounteren", &self.mcounteren)
            .field("mscratch", &self.mscratch)
            .field("mepc", &self.mepc)
            .field("mcause", &self.mcause)
            .field("mtval", &self.mtval)
            .field("menvcfg", &self.menvcfg)
            .field("mseccfg", &self.mseccfg)
            .field("mcycle", &self.mcycle)
            .field("minstret", &self.minstret)
            .field("mcounterinhibit", &self.mcounterinhibit)
            .field("[m/s]status", &self.status)
            .finish_non_exhaustive()
    }
}

impl CsrHolder {
    pub fn new(hart_id: u64, timer: TimerRef) -> Self {
        Self {
            // UserMode
            #[cfg(feature = "float")]
            fflags: FFlags::empty(),
            #[cfg(feature = "float")]
            frm: RoundingMode::ToNearestTieEven,
            timer,

            // SupervisorMode
            sie: InterruptInternal::empty(),
            stvec: TrapVector {
                mode: TrapMode::Direct,
                base: 0u64.into(),
            },
            scounteren: Counters::empty(),
            senvcfg: 0,
            sscratch: 0,
            sepc: 0u64.into(),
            scause: 0,
            stval: 0,

            siselect: 0,

            // We may unwrap because 0 is a know valid value for satp
            satp: Satp::from_bits(0).unwrap(),

            // MachineMode
            mvendorid: 0,
            marchid: 0,
            mimpid: 0,
            mhartid: hart_id,
            mconfigptr: 0,
            misa: Isa::maximal(),
            medeleg: Exception::empty(),
            mideleg: InterruptInternal::empty(),
            mie: InterruptInternal::empty(),
            mtvec: TrapVector {
                mode: TrapMode::Direct,
                base: 0u64.into(),
            },
            mcounteren: Counters::empty(),
            mscratch: 0,
            mepc: 0u64.into(),
            mcause: 0,
            mtval: 0,
            mip: Rc::new(Mutex::new(InterruptInternal::empty())),
            menvcfg: 0,
            miselect: 0,
            mseccfg: 0,
            mcycle: 0,
            minstret: 0,
            mcounterinhibit: Counters::empty(),
            pmp: PMP::default(),
            csr_providers: Vec::new(),
            status: Status {
                sie: false,
                mie: false,
                spie: false,
                mpie: false,
                spp: PrivilegeMode::User,
                vs: FloatVectorXternalStatus::Off,
                mpp: PrivilegeMode::User,
                fs: FloatVectorXternalStatus::Initial,
                xs: FloatVectorXternalStatus::Off,
                mprv: false,
                sum: false,
                mxr: false,
                tvm: false,
                tw: false,
                tsr: false,
            },
        }
    }

    pub(crate) fn add_csr_provider<P: CsrProvider + 'static>(&mut self, provider: P) {
        self.csr_providers.push(Box::new(provider));
    }

    pub(in crate::hart) fn isa(&self) -> BitFlags<Isa> {
        self.misa
    }

    pub(in crate::hart) fn inc_instret(&mut self, value: u64) {
        if !self.mcounterinhibit.contains(Counters::InstRet) {
            self.minstret += value;
        }
    }

    pub(in crate::hart) fn inc_cycle(&mut self, value: u64) {
        if !self.mcounterinhibit.contains(Counters::Cycle) {
            self.mcycle += value;
        }
    }

    pub(crate) fn get_status_mut(&mut self) -> &mut Status {
        &mut self.status
    }

    pub(crate) fn get_status(&self) -> &Status {
        &self.status
    }

    pub(crate) fn get_sepc(&self) -> Address {
        self.sepc
    }

    pub(crate) fn get_mepc(&self) -> Address {
        self.mepc
    }

    pub(crate) fn get_satp(&self) -> Satp {
        self.satp
    }

    pub(crate) fn get_mxr_sum(&self) -> (bool, bool) {
        (self.status.mxr, self.status.sum)
    }

    #[cfg(feature = "float")]
    pub(crate) fn get_frm(&self) -> RoundingMode {
        self.frm
    }

    #[cfg(feature = "float")]
    pub(crate) fn save_env_fflags(&mut self) {
        let mut flags = ExceptionFlags::default();
        flags.get();
        self.fflags = FFlags::from_bits_truncate(flags.to_bits() as u64);
    }

    #[cfg(feature = "float")]
    pub(crate) fn load_env_fflags(&self) {
        let flags = ExceptionFlags::from_bits(self.fflags.bits() as u8);
        flags.set();
    }

    fn counter_enabled(&self, privilege: PrivilegeMode, counter: CsrAddress) -> bool {
        let counter = BitFlags::<Counters>::from_bits_truncate(<u16 as From<CsrAddress>>::from(
            counter - 0xC00u16,
        ) as u32);
        (self.mcounteren.contains(counter) && privilege == PrivilegeMode::Supervisor)
            || (self.scounteren.contains(counter) && privilege == PrivilegeMode::User)
    }

    fn get_mireg(&self, reg: MiReg) -> Option<u64> {
        self.csr_providers
            .iter()
            .find(|p| p.has_sireg(reg, self.miselect))
            .and_then(|p| p.get_sireg(reg, self.miselect))
    }

    fn write_mireg(
        &mut self,
        reg: MiReg,
        value: u64,
        should_read: bool,
    ) -> Result<Option<u64>, ExecuteError> {
        self.csr_providers
            .iter_mut()
            .find(|p| p.has_mireg(reg, self.miselect))
            .map(|p| p.write_mireg(reg, self.miselect, value, should_read))
            .unwrap_or(Err(ExecuteError::Exception(Exception::IllegalInstruction)))
    }

    fn set_mireg(
        &mut self,
        reg: MiReg,
        mask: u64,
        should_write: bool,
    ) -> Result<u64, ExecuteError> {
        self.csr_providers
            .iter_mut()
            .find(|p| p.has_mireg(reg, self.miselect))
            .map(|p| p.set_mireg(reg, self.miselect, mask, should_write))
            .unwrap_or(Err(ExecuteError::Exception(Exception::IllegalInstruction)))
    }

    fn clear_mireg(
        &mut self,
        reg: MiReg,
        mask: u64,
        should_write: bool,
    ) -> Result<u64, ExecuteError> {
        self.csr_providers
            .iter_mut()
            .find(|p| p.has_mireg(reg, self.miselect))
            .map(|p| p.clear_mireg(reg, self.miselect, mask, should_write))
            .unwrap_or(Err(ExecuteError::Exception(Exception::IllegalInstruction)))
    }

    fn get_sireg(&self, reg: MiReg) -> Option<u64> {
        self.csr_providers
            .iter()
            .find(|p| p.has_sireg(reg, self.siselect))
            .and_then(|p| p.get_sireg(reg, self.siselect))
    }

    fn write_sireg(
        &mut self,
        reg: MiReg,
        value: u64,
        should_read: bool,
    ) -> Result<Option<u64>, ExecuteError> {
        self.csr_providers
            .iter_mut()
            .find(|p| p.has_sireg(reg, self.siselect))
            .map(|p| p.write_sireg(reg, self.siselect, value, should_read))
            .unwrap_or(Err(ExecuteError::Exception(Exception::IllegalInstruction)))
    }

    fn set_sireg(
        &mut self,
        reg: MiReg,
        mask: u64,
        should_write: bool,
    ) -> Result<u64, ExecuteError> {
        self.csr_providers
            .iter_mut()
            .find(|p| p.has_sireg(reg, self.siselect))
            .map(|p| p.set_sireg(reg, self.siselect, mask, should_write))
            .unwrap_or(Err(ExecuteError::Exception(Exception::IllegalInstruction)))
    }

    fn clear_sireg(
        &mut self,
        reg: MiReg,
        mask: u64,
        should_write: bool,
    ) -> Result<u64, ExecuteError> {
        self.csr_providers
            .iter_mut()
            .find(|p| p.has_sireg(reg, self.siselect))
            .map(|p| p.clear_sireg(reg, self.siselect, mask, should_write))
            .unwrap_or(Err(ExecuteError::Exception(Exception::IllegalInstruction)))
    }

    pub fn get_csr(&self, addr: CsrAddress) -> Option<u64> {
        match addr.into() {
            #[cfg(feature = "float")]
            0x001 => Some(self.fflags.bits()),
            #[cfg(feature = "float")]
            0x002 => Some(self.frm as u64),
            #[cfg(feature = "float")]
            0x003 => Some(self.fflags.bits() | ((self.frm as u64) << 5)),

            0xC00u16 => Some(self.mcycle),
            0xC01 => Some(self.timer.get_time()),
            0xC02 => Some(self.minstret),

            0x100 => Some(self.status.to_s_bits()),
            0x104 => Some(self.sie.bits()),
            0x105 => Some(self.stvec.to_bits()),
            0x106 => Some(self.scounteren.bits() as u64),
            0x10A => Some(self.senvcfg),
            0x140 => Some(self.sscratch),
            0x141 => Some(self.sepc.into()),
            0x142 => Some(self.scause),
            0x143 => Some(self.stval),
            0x144 => Some((*self.mip.lock().unwrap() & self.mideleg).bits()),
            0x150 => Some(self.siselect),
            0x151 => self.get_sireg(MiReg::MiReg1),
            0x152 => self.get_sireg(MiReg::MiReg2),
            0x153 => self.get_sireg(MiReg::MiReg3),
            0x155 => self.get_sireg(MiReg::MiReg4),
            0x156 => self.get_sireg(MiReg::MiReg5),
            0x157 => self.get_sireg(MiReg::MiReg6),
            0x180 => Some(self.satp.to_bits()),
            0xF11 => Some(self.mvendorid),
            0xF12 => Some(self.marchid),
            0xF13 => Some(self.mimpid),
            0xF14 => Some(self.mhartid),
            0xF15 => Some(self.mconfigptr),
            0x300 => Some(self.status.to_m_bits()),
            0x301 => Some(self.misa.bits() & 0b10 << 62),
            0x302 => Some(self.medeleg.bits()),
            0x303 => Some(self.mideleg.bits()),
            0x304 => Some(self.mie.bits()),
            0x305 => Some(self.mtvec.to_bits()),
            0x306 => Some(self.mcounteren.bits() as u64),
            0x320 => Some(self.mcounterinhibit.bits() as u64),
            0x340 => Some(self.mscratch),
            0x341 => Some(self.mepc.into()),
            0x342 => Some(self.mcause),
            0x343 => Some(self.mtval),
            0x344 => Some(self.mip.lock().unwrap().bits()),
            0x30A => Some(self.menvcfg),
            0x350 => Some(self.miselect),
            0x351 => self.get_mireg(MiReg::MiReg1),
            0x352 => self.get_mireg(MiReg::MiReg2),
            0x353 => self.get_mireg(MiReg::MiReg3),
            0x355 => self.get_mireg(MiReg::MiReg4),
            0x356 => self.get_mireg(MiReg::MiReg5),
            0x357 => self.get_mireg(MiReg::MiReg6),
            i @ 0x3A0..=0x3AF if i % 2 == 0 => Some(self.pmp.read_cfg_rv64((i - 0x3A0) as usize)),
            i @ 0x3B0..=0x3EF => Some(self.pmp.read_addr_rv64((i - 0x3B0) as usize)),
            0x747 => Some(self.mseccfg),
            0xB00 => Some(self.mcycle),
            0xB02 => Some(self.minstret),
            0xB03..=0xB1F => Some(0), // TODO: Performance counters
            0x323..=0x33F => Some(0), // TODO: Performance counters
            _ => self
                .csr_providers
                .iter()
                .find(|p| p.has_csr(addr))
                .and_then(|p| p.get_csr(addr)),
        }
    }

    pub fn write_csr(
        &mut self,
        addr: CsrAddress,
        value: u64,
        privilege: PrivilegeMode,
        should_read: bool,
    ) -> Result<Option<u64>, ExecuteError> {
        if addr.get_type() == CsrType::StandardRO
            || addr.get_type() == CsrType::CustomRO
            || addr.get_privilege() > privilege
            || ((0xC00u16..=0xC1F).contains(&addr.into()) && !self.counter_enabled(privilege, addr))
        {
            Err(ExecuteError::Exception(Exception::IllegalInstruction))
        } else {
            let old = self.get_csr(addr);
            match <CsrAddress as Into<u16>>::into(addr) {
                #[cfg(feature = "float")]
                0x001 => {
                    self.fflags = FFlags::from_bits_truncate(value);
                }
                #[cfg(feature = "float")]
                0x002 => {
                    self.frm = if let Ok(rm) = RoundingMode::try_from(value as u32) {
                        rm
                    } else {
                        self.frm
                    };
                }
                #[cfg(feature = "float")]
                0x003 => {
                    self.fflags = FFlags::from_bits_truncate(value);
                    self.frm = if let Ok(rm) = RoundingMode::try_from(((value >> 5) & 0b111) as u32)
                    {
                        rm
                    } else {
                        self.frm
                    };
                }
                0x100 => {
                    self.status.update_from_s_bits(value);
                }
                0x104 => {
                    self.sie =
                        BitFlags::<InterruptInternal>::from_bits_truncate(value) & self.mideleg;
                }
                0x105 => {
                    self.stvec.update_from_bits(value);
                }
                0x106 => {
                    self.scounteren =
                        BitFlags::<Counters>::from_bits_truncate((value & 0b111) as u32);
                }
                0x10A => {
                    self.senvcfg = value & 0b1;
                }
                0x140 => {
                    self.sscratch = value;
                }
                0x141 => {
                    self.sepc = (value & !0b11).into();
                }
                0x142 => {
                    self.scause = value & (0xFF | (!0 >> 1));
                }
                0x143 => {
                    self.stval = value;
                }
                0x144 => {
                    let mut mip = self.mip.lock().unwrap();
                    *mip = BitFlags::<InterruptInternal>::from_bits_truncate(
                        (mip.bits() & !(TOGGLEABLE_INTERRUPTS & S_INTERRUPT_MASK))
                            | (value & (TOGGLEABLE_INTERRUPTS & S_INTERRUPT_MASK)),
                    );
                }
                0x150 => {
                    if self.csr_providers.iter().any(|p| p.has_siselct(value)) {
                        self.siselect = value;
                    }
                }
                0x151 => return self.write_sireg(MiReg::MiReg1, value, should_read),
                0x152 => return self.write_sireg(MiReg::MiReg2, value, should_read),
                0x153 => return self.write_sireg(MiReg::MiReg3, value, should_read),
                0x155 => return self.write_sireg(MiReg::MiReg4, value, should_read),
                0x156 => return self.write_sireg(MiReg::MiReg5, value, should_read),
                0x157 => return self.write_sireg(MiReg::MiReg6, value, should_read),
                0x180 if !self.status.tvm => {
                    if let Some(val) = Satp::from_bits(value) {
                        self.satp = val;
                    }
                }
                0x300 => {
                    self.status.update_from_m_bits(value);
                }
                0x301 => {
                    self.misa = BitFlags::<Isa>::from_bits_truncate(value);
                }
                0x302 => {
                    self.medeleg = BitFlags::<Exception>::from_bits_truncate(value);
                }
                0x303 => {
                    self.mideleg =
                        BitFlags::<InterruptInternal>::from_bits_truncate(value & S_INTERRUPT_MASK);
                }
                0x304 => {
                    self.mie = BitFlags::<InterruptInternal>::from_bits_truncate(value);
                }
                0x305 => {
                    self.mtvec.update_from_bits(value);
                }
                0x306 => {
                    self.mcounteren =
                        BitFlags::<Counters>::from_bits_truncate((value & 0b111) as u32);
                }
                0x320 => {
                    self.mcounterinhibit =
                        BitFlags::<Counters>::from_bits_truncate((value & 0b101) as u32);
                }
                0x340 => {
                    self.mscratch = value;
                }
                0x341 => {
                    self.mepc = (value & !0b11).into();
                }
                0x342 => {
                    self.mcause = value & (0xFF | (!0 >> 1));
                }
                0x343 => {
                    self.mtval = value;
                }
                0x344 => {
                    // TODO: not all interrupts can be set/cleared via mip
                    let mut mip = self.mip.lock().unwrap();
                    *mip = BitFlags::<InterruptInternal>::from_bits_truncate(
                        (mip.bits() & !TOGGLEABLE_INTERRUPTS) | (value & TOGGLEABLE_INTERRUPTS),
                    );
                }
                0x350 => {
                    if self.csr_providers.iter().any(|p| p.has_miselct(value)) {
                        self.miselect = value;
                    }
                }
                0x351 => return self.write_mireg(MiReg::MiReg1, value, should_read),
                0x352 => return self.write_mireg(MiReg::MiReg2, value, should_read),
                0x353 => return self.write_mireg(MiReg::MiReg3, value, should_read),
                0x355 => return self.write_mireg(MiReg::MiReg4, value, should_read),
                0x356 => return self.write_mireg(MiReg::MiReg5, value, should_read),
                0x357 => return self.write_mireg(MiReg::MiReg6, value, should_read),
                0x30A => {
                    self.menvcfg = value & (0b1 | 0b1 << 62);
                }
                i @ 0x3A0..=0x3AF if i % 2 == 0 => {
                    self.pmp.write_cfg_rv64((i - 0x3A0) as usize, value)
                }
                i @ 0x3B0..=0x3EF => {
                    self.pmp.write_addr_rv64((i - 0x3B0) as usize, value);
                }
                0x747 => {}
                0xB00 => {
                    self.mcycle = value;
                }
                0xB02 => {
                    self.minstret = value;
                }
                0xB03..=0xB1F => {} // TODO: Performance counters
                0x323..=0x33F => {} // TODO: Performance counters
                _ => {
                    return if let Some(v) = self
                        .csr_providers
                        .iter_mut()
                        .find(|p| p.has_csr(addr))
                        .map(|p| p.write_csr(addr, value, should_read))
                    {
                        v
                    } else {
                        Err(ExecuteError::Exception(Exception::IllegalInstruction))
                    }
                }
            }
            if should_read {
                Ok(old)
            } else {
                Ok(None)
            }
        }
    }

    pub fn set_csr(
        &mut self,
        addr: CsrAddress,
        mask: u64,
        privilege: PrivilegeMode,
        should_write: bool,
    ) -> Result<u64, ExecuteError> {
        if addr.get_privilege() > privilege
            || ((addr.get_type() == CsrType::StandardRO || addr.get_type() == CsrType::CustomRO)
                && should_write)
        {
            return Err(ExecuteError::Exception(Exception::IllegalInstruction));
        }
        let old = self.get_csr(addr);
        if should_write {
            match <CsrAddress as Into<u16>>::into(addr) {
                #[cfg(feature = "float")]
                0x001 => {
                    self.fflags = FFlags::from_bits_truncate(self.fflags.bits() | mask);
                }
                #[cfg(feature = "float")]
                0x002 => {
                    self.frm = if let Ok(rm) =
                        RoundingMode::try_from((self.frm as u32) | ((mask & 0b111) as u32))
                    {
                        rm
                    } else {
                        self.frm
                    };
                }
                #[cfg(feature = "float")]
                0x003 => {
                    let value = (self.fflags.bits() | ((self.frm as u64) << 5)) | (mask & 0xFF);
                    self.fflags = FFlags::from_bits_truncate(value);
                    self.frm = if let Ok(rm) = RoundingMode::try_from(((value >> 5) & 0b111) as u32)
                    {
                        rm
                    } else {
                        self.frm
                    };
                }
                0x100 => {
                    self.status
                        .update_from_s_bits(self.status.to_s_bits() | mask);
                }
                0x104 => {
                    self.sie =
                        BitFlags::<InterruptInternal>::from_bits_truncate(self.sie.bits() | mask)
                            & self.mideleg;
                }
                0x105 => {
                    self.stvec.update_from_bits(self.stvec.to_bits() | mask);
                }
                0x106 => {
                    self.scounteren = BitFlags::<Counters>::from_bits_truncate(
                        (self.scounteren.bits() | mask as u32) & 0b111,
                    );
                }
                0x10A => {
                    self.senvcfg = (self.senvcfg | mask) & 0b1;
                }
                0x140 => {
                    self.sscratch |= mask;
                }
                0x141 => {
                    self.sepc = ((<Address as Into<u64>>::into(self.sepc) | mask) & !0b11).into();
                }
                0x142 => {
                    self.scause = (self.scause | mask) & (0xFF | (!0 >> 1));
                }
                0x143 => {
                    self.stval |= mask;
                }
                0x144 => {
                    let mut mip = self.mip.lock().unwrap();
                    *mip = BitFlags::<InterruptInternal>::from_bits_truncate(
                        mip.bits() | (mask & (TOGGLEABLE_INTERRUPTS & S_INTERRUPT_MASK)),
                    );
                }
                0x150 => {
                    let val = self.siselect | mask;
                    if self.csr_providers.iter().any(|p| p.has_siselct(val)) {
                        self.siselect = val;
                    }
                }
                0x151 => return self.set_sireg(MiReg::MiReg1, mask, should_write),
                0x152 => return self.set_sireg(MiReg::MiReg2, mask, should_write),
                0x153 => return self.set_sireg(MiReg::MiReg3, mask, should_write),
                0x155 => return self.set_sireg(MiReg::MiReg4, mask, should_write),
                0x156 => return self.set_sireg(MiReg::MiReg5, mask, should_write),
                0x157 => return self.set_sireg(MiReg::MiReg6, mask, should_write),
                0x180 if !self.status.tvm => {
                    if let Some(val) = Satp::from_bits(self.satp.to_bits() | mask) {
                        self.satp = val;
                    }
                }
                0x300 => {
                    self.status
                        .update_from_m_bits(self.status.to_m_bits() | mask);
                }
                0x301 => {
                    self.misa = BitFlags::<Isa>::from_bits_truncate(self.misa.bits() | mask);
                }
                0x302 => {
                    self.medeleg =
                        BitFlags::<Exception>::from_bits_truncate(self.medeleg.bits() | mask);
                }
                0x303 => {
                    self.mideleg = BitFlags::<InterruptInternal>::from_bits_truncate(
                        (self.mideleg.bits() | mask) & S_INTERRUPT_MASK,
                    );
                }
                0x304 => {
                    self.mie =
                        BitFlags::<InterruptInternal>::from_bits_truncate(self.mie.bits() | mask);
                }
                0x305 => {
                    self.mtvec.update_from_bits(self.mtvec.to_bits() | mask);
                }
                0x306 => {
                    self.mcounteren = BitFlags::<Counters>::from_bits_truncate(
                        (self.mcounteren.bits() | mask as u32) & 0b111,
                    );
                }
                0x320 => {
                    self.mcounterinhibit = BitFlags::<Counters>::from_bits_truncate(
                        (self.mcounterinhibit.bits() | mask as u32) & 0b101,
                    );
                }
                0x340 => {
                    self.mscratch |= mask;
                }
                0x341 => {
                    self.mepc = ((<Address as Into<u64>>::into(self.mepc) | mask) & !0b11).into();
                }
                0x342 => {
                    self.mcause = (self.mcause | mask) & (0xFF | (!0 >> 1));
                }
                0x343 => {
                    self.mtval |= mask;
                }
                0x344 => {
                    // TODO: not all interrupts can be set/cleared via mip
                    let mut mip = self.mip.lock().unwrap();
                    *mip = BitFlags::<InterruptInternal>::from_bits_truncate(
                        self.mie.bits() | (mask & TOGGLEABLE_INTERRUPTS),
                    );
                }
                0x30A => {
                    self.menvcfg = (self.menvcfg | mask) & (0b1 | 0b1 << 62);
                }
                0x350 => {
                    let val = self.miselect | mask;
                    if self.csr_providers.iter().any(|p| p.has_miselct(val)) {
                        self.miselect = val;
                    }
                }
                0x351 => return self.set_mireg(MiReg::MiReg1, mask, should_write),
                0x352 => return self.set_mireg(MiReg::MiReg2, mask, should_write),
                0x353 => return self.set_mireg(MiReg::MiReg3, mask, should_write),
                0x355 => return self.set_mireg(MiReg::MiReg4, mask, should_write),
                0x356 => return self.set_mireg(MiReg::MiReg5, mask, should_write),
                0x357 => return self.set_mireg(MiReg::MiReg6, mask, should_write),
                i @ 0x3A0..=0x3AF if i % 2 == 0 => {
                    self.pmp.write_cfg_rv64(
                        (i - 0x3A0) as usize,
                        self.pmp.read_cfg_rv64((i - 0x3A0) as usize) | mask,
                    );
                }
                i @ 0x3B0..=0x3EF => {
                    self.pmp.write_addr_rv64(
                        (i - 0x3B0) as usize,
                        self.pmp.read_addr_rv64((i - 0x3B0) as usize) | mask,
                    );
                }
                0xB00 => {
                    self.mcycle |= mask;
                }
                0xB02 => {
                    self.minstret |= mask;
                }
                0xB03..=0xB1F => {} // TODO: Performance counters
                0x323..=0x33F => {} // TODO: Performance counters
                _ => {
                    return self
                        .csr_providers
                        .iter_mut()
                        .find(|p| p.has_csr(addr))
                        .map(|p| p.set_csr(addr, mask, should_write))
                        .unwrap_or(Err(ExecuteError::Exception(Exception::IllegalInstruction)))
                }
            }
        }
        old.ok_or(ExecuteError::Exception(Exception::IllegalInstruction))
    }

    pub fn clear_csr(
        &mut self,
        addr: CsrAddress,
        mask: u64,
        privilege: PrivilegeMode,
        should_write: bool,
    ) -> Result<u64, ExecuteError> {
        if addr.get_privilege() > privilege
            || ((addr.get_type() == CsrType::StandardRO || addr.get_type() == CsrType::CustomRO)
                && should_write)
        {
            return Err(ExecuteError::Exception(Exception::IllegalInstruction));
        }
        let old = self.get_csr(addr);
        if should_write {
            match <CsrAddress as Into<u16>>::into(addr) {
                #[cfg(feature = "float")]
                0x001 => {
                    self.fflags = FFlags::from_bits_truncate(self.fflags.bits() & !mask);
                }
                #[cfg(feature = "float")]
                0x002 => {
                    self.frm = if let Ok(rm) =
                        RoundingMode::try_from((self.frm as u32) & !((mask & 0b111) as u32))
                    {
                        rm
                    } else {
                        self.frm
                    };
                }
                #[cfg(feature = "float")]
                0x003 => {
                    let value = (self.fflags.bits() | ((self.frm as u64) << 5)) & !(mask & 0xFF);
                    self.fflags = FFlags::from_bits_truncate(value);
                    self.frm = if let Ok(rm) = RoundingMode::try_from(((value >> 5) & 0b111) as u32)
                    {
                        rm
                    } else {
                        self.frm
                    };
                }
                0x100 => {
                    self.status
                        .update_from_s_bits(self.status.to_s_bits() & !mask);
                }
                0x104 => {
                    self.sie =
                        BitFlags::<InterruptInternal>::from_bits_truncate(self.sie.bits() & !mask)
                            & self.mideleg;
                }
                0x105 => {
                    self.stvec.update_from_bits(self.stvec.to_bits() & !mask);
                }
                0x106 => {
                    self.scounteren = BitFlags::<Counters>::from_bits_truncate(
                        (self.scounteren.bits() & !mask as u32) & 0b111,
                    );
                }
                0x10A => {
                    self.senvcfg = (self.senvcfg & !mask) & 0b1;
                }
                0x140 => {
                    self.sscratch &= !mask;
                }
                0x141 => {
                    self.sepc = ((<Address as Into<u64>>::into(self.sepc) & !mask) & !0b11).into();
                }
                0x142 => {
                    self.scause = (self.scause & !mask) & (0xFF | (!0 >> 1));
                }
                0x143 => {
                    self.stval &= !mask;
                }
                0x144 => {
                    let mut mip = self.mip.lock().unwrap();
                    *mip = BitFlags::<InterruptInternal>::from_bits_truncate(
                        mip.bits() & !(mask & (S_INTERRUPT_MASK & TOGGLEABLE_INTERRUPTS)),
                    );
                }
                0x150 => {
                    let val = self.siselect & !mask;
                    if self.csr_providers.iter().any(|p| p.has_siselct(val)) {
                        self.siselect = val;
                    }
                }
                0x151 => return self.clear_sireg(MiReg::MiReg1, mask, should_write),
                0x152 => return self.clear_sireg(MiReg::MiReg2, mask, should_write),
                0x153 => return self.clear_sireg(MiReg::MiReg3, mask, should_write),
                0x155 => return self.clear_sireg(MiReg::MiReg4, mask, should_write),
                0x156 => return self.clear_sireg(MiReg::MiReg5, mask, should_write),
                0x157 => return self.clear_sireg(MiReg::MiReg6, mask, should_write),
                0x180 if !self.status.tvm => {
                    if let Some(val) = Satp::from_bits(self.satp.to_bits() & !mask) {
                        self.satp = val;
                    }
                }
                0x300 => {
                    self.status
                        .update_from_m_bits(self.status.to_m_bits() & !mask);
                }
                0x301 => {
                    self.misa = BitFlags::<Isa>::from_bits_truncate(self.misa.bits() & !mask);
                }
                0x302 => {
                    self.medeleg =
                        BitFlags::<Exception>::from_bits_truncate(self.medeleg.bits() & !mask);
                }
                0x303 => {
                    self.mideleg = BitFlags::<InterruptInternal>::from_bits_truncate(
                        (self.mideleg.bits() & !mask) & S_INTERRUPT_MASK,
                    );
                }
                0x304 => {
                    self.mie =
                        BitFlags::<InterruptInternal>::from_bits_truncate(self.mie.bits() & !mask);
                }
                0x305 => {
                    self.mtvec.update_from_bits(self.mtvec.to_bits() & !mask);
                }
                0x306 => {
                    self.mcounteren = BitFlags::<Counters>::from_bits_truncate(
                        (self.mcounteren.bits() & !mask as u32) & 0b111,
                    );
                }
                0x320 => {
                    self.mcounterinhibit = BitFlags::<Counters>::from_bits_truncate(
                        (self.mcounterinhibit.bits() & !mask as u32) & 0b111,
                    );
                }
                0x340 => {
                    self.mscratch &= !mask;
                }
                0x341 => {
                    self.mepc = ((<Address as Into<u64>>::into(self.mepc) & !mask) & !0b11).into();
                }
                0x342 => {
                    self.mcause = (self.mcause & !mask) & (0xFF | (!0 >> 1));
                }
                0x343 => {
                    self.mtval &= !mask;
                }
                0x344 => {
                    // TODO: not all interrupts can be set/cleared via mip
                    let mut mip = self.mip.lock().unwrap();
                    *mip = BitFlags::<InterruptInternal>::from_bits_truncate(
                        self.mie.bits() | (mask & TOGGLEABLE_INTERRUPTS),
                    );
                }
                0x30A => {
                    self.menvcfg = (self.menvcfg & !mask) & (0b1 | 0b1 << 62);
                }
                0x350 => {
                    let val = self.miselect & !mask;
                    if self.csr_providers.iter().any(|p| p.has_miselct(val)) {
                        self.miselect = val;
                    }
                }
                0x351 => return self.clear_mireg(MiReg::MiReg1, mask, should_write),
                0x352 => return self.clear_mireg(MiReg::MiReg2, mask, should_write),
                0x353 => return self.clear_mireg(MiReg::MiReg3, mask, should_write),
                0x355 => return self.clear_mireg(MiReg::MiReg4, mask, should_write),
                0x356 => return self.clear_mireg(MiReg::MiReg5, mask, should_write),
                0x357 => return self.clear_mireg(MiReg::MiReg6, mask, should_write),
                i @ 0x3A0..=0x3AF if i % 2 == 0 => {
                    self.pmp.write_cfg_rv64(
                        (i - 0x3A0) as usize,
                        self.pmp.read_cfg_rv64((i - 0x3A0) as usize) & !mask,
                    );
                }
                i @ 0x3B0..=0x3EF => {
                    self.pmp.write_addr_rv64(
                        (i - 0x3B0) as usize,
                        self.pmp.read_addr_rv64((i - 0x3B0) as usize) & !mask,
                    );
                }
                0xB00 => {
                    self.mcycle &= !mask;
                }
                0xB02 => {
                    self.minstret &= !mask;
                }
                0xB03..=0xB1F => {} // TODO: Performance counters
                0x323..=0x33F => {} // TODO: Performance counters
                _ => {
                    return self
                        .csr_providers
                        .iter_mut()
                        .find(|p| p.has_csr(addr))
                        .map(|p| p.clear_csr(addr, mask, should_write))
                        .unwrap_or(Err(ExecuteError::Exception(Exception::IllegalInstruction)))
                }
            }
        }
        old.ok_or(ExecuteError::Exception(Exception::IllegalInstruction))
    }
}

impl TrapVector {
    fn update_from_bits(&mut self, bits: u64) {
        match bits & 0b11 {
            0 => self.mode = TrapMode::Direct,
            1 => self.mode = TrapMode::Vectored,
            _ => return,
        }

        self.base = (bits & !0b11).into();
    }

    fn to_bits(&self) -> u64 {
        match self.mode {
            TrapMode::Direct => self.base.into(),
            TrapMode::Vectored => <Address as Into<u64>>::into(self.base) | 1,
        }
    }
}

impl Status {
    fn to_s_bits(&self) -> u64 {
        let mut bits = 0;

        if self.sie {
            bits |= 0b1 << 1;
        }

        if self.spie {
            bits |= 0b1 << 5;
        }

        bits |= (self.spp as u64 & 0b1) << 8;

        bits |= (self.vs as u64 & 0b11) << 9;
        bits |= (self.fs as u64 & 0b11) << 13;
        bits |= (self.xs as u64 & 0b11) << 15;

        if self.sum {
            bits |= 0b1 << 18;
        }

        if self.mxr {
            bits |= 0b1 << 19;
        }

        bits |= 0b10 << 32; // UXL Needs to be 10 for 64 bit

        if self.vs == FloatVectorXternalStatus::Dirty
            || self.fs == FloatVectorXternalStatus::Dirty
            || self.xs == FloatVectorXternalStatus::Dirty
        {
            bits |= 0b1 << 63;
        }

        bits
    }

    fn update_from_s_bits(&mut self, bits: u64) {
        self.sie = (bits & (0b1 << 1)) > 0;

        self.spie = (bits & (0b1 << 5)) > 0;

        if (bits & (0b1 << 8)) > 0 {
            self.spp = PrivilegeMode::Supervisor;
        } else {
            self.spp = PrivilegeMode::User;
        }

        match bits >> 9 & 0b11 {
            0b00 => self.vs = FloatVectorXternalStatus::Off,
            0b01 => self.vs = FloatVectorXternalStatus::Initial,
            0b10 => self.vs = FloatVectorXternalStatus::Clean,
            0b11 => self.vs = FloatVectorXternalStatus::Dirty,
            _ => unreachable!(),
        }

        match bits >> 13 & 0b11 {
            0b00 => self.fs = FloatVectorXternalStatus::Off,
            0b01 => self.fs = FloatVectorXternalStatus::Initial,
            0b10 => self.fs = FloatVectorXternalStatus::Clean,
            0b11 => self.fs = FloatVectorXternalStatus::Dirty,
            _ => unreachable!(),
        }

        match bits >> 15 & 0b11 {
            0b00 => self.xs = FloatVectorXternalStatus::Off,
            0b01 => self.xs = FloatVectorXternalStatus::Initial,
            0b10 => self.xs = FloatVectorXternalStatus::Clean,
            0b11 => self.xs = FloatVectorXternalStatus::Dirty,
            _ => unreachable!(),
        }

        self.sum = (bits & (0b1 << 18)) > 0;

        self.mxr = (bits & (0b1 << 19)) > 0;
    }

    fn to_m_bits(&self) -> u64 {
        let mut bits = 0;

        if self.sie {
            bits |= 0b1 << 1;
        }

        if self.mie {
            bits |= 0b1 << 3;
        }

        if self.spie {
            bits |= 0b1 << 5;
        }

        if self.mpie {
            bits |= 0b1 << 7;
        }

        bits |= (self.spp as u64 & 0b1) << 8;
        bits |= (self.vs as u64 & 0b11) << 9;
        bits |= (self.mpp as u64 & 0b11) << 11;
        bits |= (self.fs as u64 & 0b11) << 13;
        bits |= (self.xs as u64 & 0b11) << 15;

        if self.mprv {
            bits |= 0b1 << 17
        }

        if self.sum {
            bits |= 0b1 << 18;
        }

        if self.mxr {
            bits |= 0b1 << 19;
        }

        if self.tvm {
            bits |= 0b1 << 20;
        }

        if self.tw {
            bits |= 0b1 << 21;
        }

        if self.tsr {
            bits |= 0b1 << 22;
        }

        bits |= 0b10 << 32; // UXL Needs to be 10 for 64 bit.
        bits |= 0b10 << 34; // SXL Needs to be 10 for 64 bit

        if self.vs == FloatVectorXternalStatus::Dirty
            || self.fs == FloatVectorXternalStatus::Dirty
            || self.xs == FloatVectorXternalStatus::Dirty
        {
            bits |= 0b1 << 63;
        }

        bits
    }

    fn update_from_m_bits(&mut self, bits: u64) {
        self.sie = (bits & (0b1 << 1)) > 0;

        self.mie = (bits & (0b1 << 3)) > 0;

        self.spie = (bits & (0b1 << 5)) > 0;

        self.mpie = (bits & (0b1 << 7)) > 0;

        if (bits & (0b1 << 8)) > 0 {
            self.spp = PrivilegeMode::Supervisor;
        } else {
            self.spp = PrivilegeMode::User;
        }

        match bits >> 9 & 0b11 {
            0b00 => self.vs = FloatVectorXternalStatus::Off,
            0b01 => self.vs = FloatVectorXternalStatus::Initial,
            0b10 => self.vs = FloatVectorXternalStatus::Clean,
            0b11 => self.vs = FloatVectorXternalStatus::Dirty,
            _ => unreachable!(),
        }

        match (bits & (0b11 << 11)) >> 11 {
            0b00 => self.mpp = PrivilegeMode::User,
            0b01 => self.mpp = PrivilegeMode::Supervisor,
            0b11 => self.mpp = PrivilegeMode::Machine,
            _ => unreachable!(),
        }

        match bits >> 13 & 0b11 {
            0b00 => self.fs = FloatVectorXternalStatus::Off,
            0b01 => self.fs = FloatVectorXternalStatus::Initial,
            0b10 => self.fs = FloatVectorXternalStatus::Clean,
            0b11 => self.fs = FloatVectorXternalStatus::Dirty,
            _ => unreachable!(),
        }

        match bits >> 15 & 0b11 {
            0b00 => self.xs = FloatVectorXternalStatus::Off,
            0b01 => self.xs = FloatVectorXternalStatus::Initial,
            0b10 => self.xs = FloatVectorXternalStatus::Clean,
            0b11 => self.xs = FloatVectorXternalStatus::Dirty,
            _ => unreachable!(),
        }

        self.mprv = (bits & (0b1 << 17)) > 0;

        self.sum = (bits & (0b1 << 18)) > 0;

        self.mxr = (bits & (0b1 << 19)) > 0;

        self.tvm = (bits & (0b1 << 20)) > 0;

        self.tw = (bits & (0b1 << 21)) > 0;

        self.tsr = (bits & (0b1 << 22)) > 0;
    }
}
