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
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord)]
pub enum InterruptInternal {
    SupervisorSoftware = 0b1 << 1,
    MachineSoftware = 0b1 << 3,
    SupervisorTimer = 0b1 << 5,
    MachineTimer = 0b1 << 7,
    SupervisorExternal = 0b1 << 9,
    MachineExternal = 0b1 << 11,
}

impl PartialOrd for InterruptInternal {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self {
            InterruptInternal::MachineExternal => match other {
                Self::MachineExternal => Some(std::cmp::Ordering::Equal),
                _ => Some(std::cmp::Ordering::Greater),
            },
            InterruptInternal::MachineSoftware => match other {
                Self::MachineExternal => Some(std::cmp::Ordering::Greater),
                Self::MachineSoftware => Some(std::cmp::Ordering::Equal),
                _ => Some(std::cmp::Ordering::Less),
            },
            InterruptInternal::MachineTimer => match other {
                Self::MachineExternal | Self::MachineSoftware => Some(std::cmp::Ordering::Greater),
                Self::MachineTimer => Some(std::cmp::Ordering::Equal),
                _ => Some(std::cmp::Ordering::Less),
            },
            InterruptInternal::SupervisorExternal => match other {
                Self::MachineExternal | Self::MachineSoftware | Self::MachineTimer => {
                    Some(std::cmp::Ordering::Greater)
                }
                Self::SupervisorExternal => Some(std::cmp::Ordering::Equal),
                _ => Some(std::cmp::Ordering::Less),
            },
            InterruptInternal::SupervisorSoftware => match other {
                Self::MachineExternal
                | Self::MachineSoftware
                | Self::MachineTimer
                | InterruptInternal::SupervisorExternal => Some(std::cmp::Ordering::Greater),
                Self::SupervisorSoftware => Some(std::cmp::Ordering::Equal),
                _ => Some(std::cmp::Ordering::Less),
            },

            InterruptInternal::SupervisorTimer => match other {
                Self::MachineExternal
                | Self::MachineSoftware
                | Self::MachineTimer
                | InterruptInternal::SupervisorExternal
                | InterruptInternal::SupervisorSoftware => Some(std::cmp::Ordering::Greater),
                Self::SupervisorTimer => Some(std::cmp::Ordering::Equal),
                _ => Some(std::cmp::Ordering::Less),
            },
        }
    }
}

impl InterruptInternal {
    pub fn get_code(&self) -> u64 {
        match self {
            InterruptInternal::SupervisorSoftware => 1,
            InterruptInternal::MachineSoftware => 3,
            InterruptInternal::SupervisorTimer => 5,
            InterruptInternal::MachineTimer => 7,
            InterruptInternal::SupervisorExternal => 9,
            InterruptInternal::MachineExternal => 11,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Interrupt {
    SSoftware,
    MSoftware,
    Timer,
    External,
}

#[derive(Debug)]
pub enum InterruptTarget {
    All,
    Single(usize),
}

#[derive(Debug)]
pub enum TrapCause {
    Exception(Exception),
    Interrupt(InterruptInternal),
}
