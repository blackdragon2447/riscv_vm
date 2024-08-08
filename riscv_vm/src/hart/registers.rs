use std::fmt::Debug;

#[cfg(feature = "float")]
use softfloat_wrapper::{Float, F32};

pub struct Registers {
    int_registers: [i64; 32],
    #[cfg(feature = "float")]
    float_registers: [u64; 32],
}

impl Debug for Registers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut debug = f.debug_struct("Registers");

        for (i, r) in self.int_registers.iter().enumerate() {
            // debug.field(format!("X{i}").as_str(), &format!("{r:#X}"));
            debug.field(format!("X{i}").as_str(), &format!("{r:#X}"));
        }

        debug.field("", &"");

        #[cfg(feature = "float")]
        for (i, r) in self.float_registers.iter().enumerate() {
            debug.field(format!("F{i}").as_str(), &format!("{r:#X}"));
        }

        debug.finish()
    }
}

impl Default for Registers {
    fn default() -> Self {
        Self::new()
    }
}

impl Registers {
    pub fn new() -> Self {
        Self {
            int_registers: [0; 32],
            #[cfg(feature = "float")]
            float_registers: [0; 32],
        }
    }

    pub fn get_int(&self, register: IntRegister) -> i64 {
        self.int_registers[register as usize]
    }

    pub fn set_int(&mut self, register: IntRegister, value: i64) {
        match register {
            IntRegister::X0 => {}
            register => self.int_registers[register as usize] = value,
        }
    }

    #[cfg(feature = "float")]
    pub fn get_f32(&self, register: FloatRegister) -> F32 {
        F32::from_bits(self.float_registers[register as usize] as u32)
    }

    #[cfg(feature = "float")]
    pub fn set_f32(&mut self, register: FloatRegister, value: F32) {
        self.float_registers[register as usize] = value.to_bits() as u64;
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum IntRegister {
    X0,
    X1,
    X2,
    X3,
    X4,
    X5,
    X6,
    X7,
    X8,
    X9,
    X10,
    X11,
    X12,
    X13,
    X14,
    X15,
    X16,
    X17,
    X18,
    X19,
    X20,
    X21,
    X22,
    X23,
    X24,
    X25,
    X26,
    X27,
    X28,
    X29,
    X30,
    X31,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PCRegister;

impl From<u32> for IntRegister {
    fn from(value: u32) -> Self {
        match value {
            0 => IntRegister::X0,
            1 => IntRegister::X1,
            2 => IntRegister::X2,
            3 => IntRegister::X3,
            4 => IntRegister::X4,
            5 => IntRegister::X5,
            6 => IntRegister::X6,
            7 => IntRegister::X7,
            8 => IntRegister::X8,
            9 => IntRegister::X9,
            10 => IntRegister::X10,
            11 => IntRegister::X11,
            12 => IntRegister::X12,
            13 => IntRegister::X13,
            14 => IntRegister::X14,
            15 => IntRegister::X15,
            16 => IntRegister::X16,
            17 => IntRegister::X17,
            18 => IntRegister::X18,
            19 => IntRegister::X19,
            20 => IntRegister::X20,
            21 => IntRegister::X21,
            22 => IntRegister::X22,
            23 => IntRegister::X23,
            24 => IntRegister::X24,
            25 => IntRegister::X25,
            26 => IntRegister::X26,
            27 => IntRegister::X27,
            28 => IntRegister::X28,
            29 => IntRegister::X29,
            30 => IntRegister::X30,
            31 => IntRegister::X31,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
#[cfg(feature = "float")]
pub enum FloatRegister {
    F0,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,
    F25,
    F26,
    F27,
    F28,
    F29,
    F30,
    F31,
}

#[cfg(feature = "float")]
impl From<u32> for FloatRegister {
    fn from(value: u32) -> Self {
        match value {
            0 => FloatRegister::F0,
            1 => FloatRegister::F1,
            2 => FloatRegister::F2,
            3 => FloatRegister::F3,
            4 => FloatRegister::F4,
            5 => FloatRegister::F5,
            6 => FloatRegister::F6,
            7 => FloatRegister::F7,
            8 => FloatRegister::F8,
            9 => FloatRegister::F9,
            10 => FloatRegister::F10,
            11 => FloatRegister::F11,
            12 => FloatRegister::F12,
            13 => FloatRegister::F13,
            14 => FloatRegister::F14,
            15 => FloatRegister::F15,
            16 => FloatRegister::F16,
            17 => FloatRegister::F17,
            18 => FloatRegister::F18,
            19 => FloatRegister::F19,
            20 => FloatRegister::F20,
            21 => FloatRegister::F21,
            22 => FloatRegister::F22,
            23 => FloatRegister::F23,
            24 => FloatRegister::F24,
            25 => FloatRegister::F25,
            26 => FloatRegister::F26,
            27 => FloatRegister::F27,
            28 => FloatRegister::F28,
            29 => FloatRegister::F29,
            30 => FloatRegister::F30,
            31 => FloatRegister::F31,
            _ => unreachable!(),
        }
    }
}
