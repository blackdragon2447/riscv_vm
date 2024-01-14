use enumflags2::bitflags;

#[repr(u64)]
#[bitflags]
#[derive(Clone, Copy, Debug)]
pub enum Exception {
    InstructionAddressMisaligned = 0b1 << 0,
    InstructionAccessFault = 0b1 << 1,
    IllegalInstruction = 0b1 << 2,
    BreakPoint = 0b1 << 3,
    LoadAddressMisaligned = 0b1 << 4,
    LoadAccessFault = 0b1 << 5,
    StoreAddressMisaligned = 0b1 << 6,
    StoreAccessFault = 0b1 << 7,
    EcallUMode = 0b1 << 8,
    EcallSMode = 0b1 << 9,
    EcallMMode = 0b1 << 11,
    InstructionPageFault = 0b1 << 12,
    LoadPageFault = 0b1 << 13,
    StorePageFault = 0b1 << 15,
}

impl Exception {
    pub fn get_code(&self) -> u64 {
        match self {
            Exception::InstructionAddressMisaligned => 0,
            Exception::InstructionAccessFault => 1,
            Exception::IllegalInstruction => 2,
            Exception::BreakPoint => 3,
            Exception::LoadAddressMisaligned => 4,
            Exception::LoadAccessFault => 5,
            Exception::StoreAddressMisaligned => 6,
            Exception::StoreAccessFault => 7,
            Exception::EcallUMode => 8,
            Exception::EcallSMode => 9,
            Exception::EcallMMode => 11,
            Exception::InstructionPageFault => 12,
            Exception::LoadPageFault => 13,
            Exception::StorePageFault => 15,
        }
    }
}

#[repr(u64)]
#[bitflags]
#[derive(Clone, Copy, Debug)]
pub(super) enum Interrupt {
    SupervisorSoftware = 0b1 << 1,
    MachineSoftware = 0b1 << 3,
    SupervisorTimer = 0b1 << 5,
    MachineTimer = 0b1 << 6,
    SupervisorExternal = 0b1 << 9,
    MachineExternal = 0b1 << 11,
}
