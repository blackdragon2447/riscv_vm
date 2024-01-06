#[derive(Debug)]
pub struct Registers {
    registers: [i64; 32],
}

impl Registers {
    pub fn new() -> Self {
        Self { registers: [0; 32] }
    }

    pub fn get(&self, register: IntRegister) -> i64 {
        self.registers[register as usize]
    }

    pub fn set(&mut self, register: IntRegister, value: i64) {
        match register {
            IntRegister::X0 => {}
            register => self.registers[register as usize] = value,
        }
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
