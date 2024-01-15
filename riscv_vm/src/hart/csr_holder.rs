use enumflags2::{BitFlag, BitFlags};

use crate::{execute::ExecuteError, memory::address::Address};

use super::{
    csr_address::CsrType,
    isa::Isa,
    privilege::{self, PrivilegeMode},
    trap::{Exception, Interrupt},
    CsrAddress,
};
use std::{collections::HashMap, time::Instant};

#[derive(Debug)]
pub struct CsrHolder {
    // UserMode
    // cycle (tied to mcylce)
    time_started: Instant,
    // instret (tied to minstret)

    // SupervisorMode
    // sstatus (tied to status)
    // sie: u64,
    pub(in crate::hart) stvec: Address,
    scounteren: u64,
    senvcfg: u64,
    sscratch: u64,
    pub(in crate::hart) sepc: Address,
    pub(in crate::hart) scause: u64,
    pub(in crate::hart) stval: u64,
    // sip: u64
    // satp

    // MachineMode
    mvendorid: u64,
    marchid: u64,
    mimpid: u64,
    mhartid: u64,
    mconfigptr: u64,
    misa: BitFlags<Isa>,
    pub(in crate::hart) medeleg: BitFlags<Exception>,
    mideleg: BitFlags<Interrupt>,
    // mie: u64,
    pub(in crate::hart) mtvec: Address,
    mcounteren: u64,
    mscratch: u64,
    pub(in crate::hart) mepc: Address,
    pub(in crate::hart) mcause: u64,
    pub(in crate::hart) mtval: u64,
    // mip: u64
    menvcfg: u64,
    mseccfg: u64,

    mcycle: u64,
    minstret: u64,
    mcounterinhibit: u64,
    csr: HashMap<CsrAddress, u64>,

    //Other
    pub(in crate::hart) status: Status,
}

#[derive(Debug)]
pub struct Status {
    /// [1] M + S
    pub(crate) sie: bool, // always false (interrupts not impl)

    /// [3] M
    pub(crate) mie: bool, // always false (interrupts not impl)

    /// [5] M + S
    pub(crate) spie: bool,

    /// [7] M
    pub(crate) mpie: bool,

    /// [8] M + S
    pub(crate) spp: PrivilegeMode,

    /// [11..12] M
    pub(crate) mpp: PrivilegeMode,

    /// [17] M
    pub(crate) mprv: bool, // always false (virt mem not impl)

    /// [18] M + S
    pub(crate) sum: bool,

    /// [19] M + S
    pub(crate) mxr: bool,

    /// [20] M
    pub(crate) tvm: bool,

    /// [21] M
    pub(crate) tw: bool,

    /// [22] M
    pub(crate) tsr: bool,
}

impl CsrHolder {
    pub fn new(hart_id: u64) -> Self {
        Self {
            // UserMode
            time_started: Instant::now(),

            // SupervisorMode
            stvec: 0u64.into(),
            scounteren: 0,
            senvcfg: 0,
            sscratch: 0,
            sepc: 0u64.into(),
            scause: 0,
            stval: 0,

            // MachineMode
            mvendorid: 0,
            marchid: 0,
            mimpid: 0,
            mhartid: hart_id,
            mconfigptr: 0,
            misa: Isa::maximal(),
            medeleg: Exception::empty(),
            mideleg: Interrupt::empty(),
            mtvec: 0u64.into(),
            mcounteren: 0,
            mscratch: 0,
            mepc: 0u64.into(),
            mcause: 0,
            mtval: 0,
            menvcfg: 0,
            mseccfg: 0,
            mcycle: 0,
            minstret: 0,
            mcounterinhibit: 0,
            csr: HashMap::new(),
            status: Status {
                sie: false,
                mie: false,
                spie: false,
                mpie: false,
                spp: PrivilegeMode::User,
                mpp: PrivilegeMode::User,
                mprv: false,
                sum: false,
                mxr: false,
                tvm: false,
                tw: false,
                tsr: false,
            },
        }
    }

    pub(in crate::hart) fn isa(&self) -> BitFlags<Isa> {
        self.misa
    }

    pub(in crate::hart) fn inc_instret(&mut self, value: u64) {
        self.minstret += value;
    }

    pub(in crate::hart) fn inc_cycle(&mut self, value: u64) {
        self.mcycle += value;
    }

    pub(crate) fn status_mut(&mut self) -> &mut Status {
        &mut self.status
    }

    pub(crate) fn get_sepc(&self) -> Address {
        self.sepc
    }

    pub(crate) fn get_mepc(&self) -> Address {
        self.mepc
    }

    pub fn get_csr(&self, addr: CsrAddress) -> u64 {
        match addr.into() {
            0xC00u16 => self.mcycle,
            0xC01 => self.time_started.elapsed().as_millis() as u64,
            0xC02 => self.minstret,

            0x100 => self.status.to_s_bits(),
            0x104 => 0, // self.sie
            0x105 => self.stvec.into(),
            0x106 => self.scounteren,
            0x10A => self.senvcfg,
            0x140 => self.sscratch,
            0x141 => self.sepc.into(),
            0x142 => self.scause,
            0x143 => self.stval,
            0x144 => 0, // self.sip
            // 0x180 => self.satp,
            0xF11 => self.mvendorid,
            0xF12 => self.marchid,
            0xF13 => self.mimpid,
            0xF14 => self.mhartid,
            0xF15 => self.mconfigptr,
            0x300 => self.status.to_m_bits(),
            0x301 => self.misa.bits() & 0b10 << 62,
            0x302 => self.medeleg.bits(),
            0x303 => self.mideleg.bits(),
            0x304 => 0, // self.mie
            0x305 => self.mtvec.into(),
            0x306 => self.mcounteren,
            0x340 => self.mscratch,
            0x341 => self.mepc.into(),
            0x342 => self.mcause,
            0x343 => self.mtval,
            0x344 => 0, // self.mip
            0x30A => self.menvcfg,
            0x747 => self.mseccfg,
            0xB00 => self.mcycle,
            0xB02 => self.minstret,
            _ => *self.csr.get(&addr).unwrap_or(&0),
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
        {
            eprintln!(
                "Illegal write to {:?}, with mode {:?} (self: {:?}), and type {:?}",
                addr,
                addr.get_privilege(),
                privilege,
                addr.get_type()
            );
            Err(ExecuteError::Exception(Exception::IllegalInstruction))
        } else {
            let old = self.get_csr(addr);
            match <CsrAddress as Into<u16>>::into(addr) {
                0x100 => {
                    self.status.update_from_s_bits(value);
                }
                0x105 => {
                    self.stvec = (value & !0b11).into();
                }
                0x106 => {
                    self.scounteren = (value & 0b111);
                }
                0x10A => {
                    self.senvcfg = (value & 0b1);
                }
                0x140 => {
                    self.sscratch = value;
                }
                0x141 => {
                    self.sepc = (value & !0b11).into();
                }
                0x142 => {
                    self.scause = (value & (0xFF | (!0 >> 1)));
                }
                0x143 => {
                    self.stval = value;
                }
                // 0x180 => self.satp,
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
                    self.mideleg = BitFlags::<Interrupt>::from_bits_truncate(value);
                }
                0x305 => {
                    self.mtvec = (value & !0b11).into();
                }
                0x306 => {
                    self.mcounteren = (value & 0b111);
                }
                0x340 => {
                    self.mscratch = value;
                }
                0x341 => {
                    self.mepc = (value & !0b11).into();
                }
                0x342 => {
                    self.mcause = (value & (0xFF | (!0 >> 1)));
                }
                0x343 => {
                    self.mtval = value;
                }
                0x30A => {
                    self.menvcfg = (value & (0b1 | 0b1 << 62));
                }
                0x747 => {}
                0xB00 => {
                    self.mcycle = value;
                }
                0xB02 => {
                    self.minstret = value;
                }
                _ => {}
            }
            if should_read {
                Ok(Some(old))
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
            eprintln!(
                "Illegal write to {:?}, with mode {:?} (self: {:?}), and type {:?}",
                addr,
                addr.get_privilege(),
                privilege,
                addr.get_type()
            );
            return Err(ExecuteError::Exception(Exception::IllegalInstruction));
        }
        let old = self.get_csr(addr);
        if should_write {
            match <CsrAddress as Into<u16>>::into(addr) {
                0x100 => {
                    self.status
                        .update_from_s_bits(self.status.to_s_bits() | mask);
                }
                0x105 => {
                    self.stvec = ((<Address as Into<u64>>::into(self.stvec) | mask) & !0b11).into();
                }
                0x106 => {
                    self.scounteren = ((self.scounteren | mask) & 0b111);
                }
                0x10A => {
                    self.senvcfg = ((self.senvcfg | mask) & 0b1);
                }
                0x140 => {
                    self.sscratch |= mask;
                }
                0x141 => {
                    self.sepc = ((<Address as Into<u64>>::into(self.sepc) | mask) & !0b11).into();
                }
                0x142 => {
                    self.scause = ((self.scause | mask) & (0xFF | (!0 >> 1)));
                }
                0x143 => {
                    self.stval |= mask;
                }
                // 0x180 => self.satp,
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
                    self.mideleg =
                        BitFlags::<Interrupt>::from_bits_truncate(self.mideleg.bits() | mask);
                }
                0x305 => {
                    self.mtvec = ((<Address as Into<u64>>::into(self.mtvec) | mask) & !0b11).into();
                }
                0x306 => {
                    self.mcounteren = ((self.mcounteren | mask) & 0b111);
                }
                0x340 => {
                    self.mscratch |= mask;
                }
                0x341 => {
                    self.mepc = ((<Address as Into<u64>>::into(self.mepc) | mask) & !0b11).into();
                }
                0x342 => {
                    self.mcause = ((self.mcause | mask) & (0xFF | (!0 >> 1)));
                }
                0x343 => {
                    self.mtval |= mask;
                }
                0x30A => {
                    self.menvcfg = ((self.menvcfg | mask) & (0b1 | 0b1 << 62));
                }
                0xB00 => {
                    self.mcycle |= mask;
                }
                0xB02 => {
                    self.minstret |= mask;
                }
                _ => {}
            }
        }
        Ok(old)
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
            eprintln!(
                "Illegal write to {:?}, with mode {:?} (self: {:?}), and type {:?}",
                addr,
                addr.get_privilege(),
                privilege,
                addr.get_type()
            );
            return Err(ExecuteError::Exception(Exception::IllegalInstruction));
        }
        let old = self.get_csr(addr);
        if should_write {
            match <CsrAddress as Into<u16>>::into(addr) {
                0x100 => {
                    self.status
                        .update_from_s_bits(self.status.to_s_bits() & !mask);
                }
                0x105 => {
                    self.stvec =
                        ((<Address as Into<u64>>::into(self.stvec) & !mask) & !0b11).into();
                }
                0x106 => {
                    self.scounteren = ((self.scounteren & !mask) & 0b111);
                }
                0x10A => {
                    self.senvcfg = ((self.senvcfg & !mask) & 0b1);
                }
                0x140 => {
                    self.sscratch &= !mask;
                }
                0x141 => {
                    self.sepc = ((<Address as Into<u64>>::into(self.sepc) & !mask) & !0b11).into();
                }
                0x142 => {
                    self.scause = ((self.scause & !mask) & (0xFF | (!0 >> 1)));
                }
                0x143 => {
                    self.stval &= !mask;
                }
                // 0x180 => self.satp,
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
                    self.mideleg =
                        BitFlags::<Interrupt>::from_bits_truncate(self.mideleg.bits() & !mask);
                }
                0x305 => {
                    self.mtvec =
                        ((<Address as Into<u64>>::into(self.mtvec) & !mask) & !0b11).into();
                }
                0x306 => {
                    self.mcounteren = ((self.mcounteren & !mask) & 0b111);
                }
                0x340 => {
                    self.mscratch &= !mask;
                }
                0x341 => {
                    self.mepc = ((<Address as Into<u64>>::into(self.mepc) & !mask) & !0b11).into();
                }
                0x342 => {
                    self.mcause = ((self.mcause & !mask) & (0xFF | (!0 >> 1)));
                }
                0x343 => {
                    self.mtval &= !mask;
                }
                0x30A => {
                    self.menvcfg = ((self.menvcfg & !mask) & (0b1 | 0b1 << 62));
                }
                0xB00 => {
                    self.mcycle &= !mask;
                }
                0xB02 => {
                    self.minstret &= !mask;
                }
                _ => {}
            }
        }
        Ok(old)
    }
}

impl Status {
    fn to_s_bits(&self) -> u64 {
        let mut bits = 0;

        if self.sie {
            bits |= (0b1 << 1);
        }

        if self.spie {
            bits |= (0b1 << 5);
        }

        bits |= ((self.spp as u64 & 0b1) << 8);

        if self.sum {
            bits |= (0b1 << 18);
        }

        if self.mxr {
            bits |= (0b1 << 19);
        }

        bits |= (0b10 << 32); // UXL Needs to be 10 for 64 bit

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

        self.sum = (bits & (0b1 << 18)) > 0;

        self.mxr = (bits & (0b1 << 19)) > 0;
    }

    fn to_m_bits(&self) -> u64 {
        let mut bits = 0;

        if self.sie {
            bits |= (0b1 << 1);
        }

        if self.mie {
            bits |= (0b1 << 3);
        }

        if self.spie {
            bits |= (0b1 << 5);
        }

        if self.mpie {
            bits |= (0b1 << 7);
        }

        bits |= ((self.spp as u64 & 0b1) << 8);
        bits |= ((self.mpp as u64 & 0b11) << 11);

        if self.mprv {
            bits |= (0b1 << 17)
        }

        if self.sum {
            bits |= (0b1 << 18);
        }

        if self.mxr {
            bits |= (0b1 << 19);
        }

        if self.tvm {
            bits |= (0b1 << 20);
        }

        if self.tw {
            bits |= (0b1 << 21);
        }

        if self.tsr {
            bits |= (0b1 << 22);
        }

        bits |= (0b10 << 32); // UXL Needs to be 10 for 64 bit.

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

        match (bits & (0b11 << 11)) >> 11 {
            0b00 => self.mpp = PrivilegeMode::User,
            0b01 => self.mpp = PrivilegeMode::Supervisor,
            0b11 => self.mpp = PrivilegeMode::Machine,
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
