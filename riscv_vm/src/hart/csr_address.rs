use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Sub},
};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CsrAddress(u16);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CsrType {
    StandardRW,
    CustomRW,
    StandardRO,
    CustomRO,
}

impl Debug for CsrAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#4X}", self.0)
    }
}

impl CsrAddress {
    pub const fn new(addr: u16) -> Self {
        Self(addr & 0xFFF)
    }

    pub fn get_type(&self) -> CsrType {
        match (
            (self.0 & 0b110000000000) >> 10,
            (self.0 & 0b001100000000) >> 8,
            (self.0 & 0b000011110000) >> 4,
        ) {
            (0b00, 0b00, _) => CsrType::StandardRW,
            (0b01, 0b00, _) => CsrType::StandardRW,
            (0b10, 0b00, _) => CsrType::CustomRO,
            (0b11, 0b00, i) if (i & 0b1000) == 0b0000 || (i & 0b1100) == 0b1000 => {
                CsrType::StandardRO
            }
            (0b11, 0b00, i) if (i & 0b1100) == 0b1100 => CsrType::CustomRO,

            (0b00, 0b11, _) => CsrType::StandardRW,
            (0b01, 0b11, i) if (i & 0b1000) == 0b0000 || (i & 0b1110) == 0b1000 => {
                CsrType::StandardRW
            }
            (0b01, 0b11, i) if (i & 0b1100) == 0b1100 => CsrType::CustomRW,
            (0b10, 0b11, i) if (i & 0b1000) == 0b0000 || (i & 0b1100) == 0b1000 => {
                CsrType::StandardRW
            }
            (0b10, 0b11, i) if (i & 0b1100) == 0b1100 => CsrType::CustomRW,
            (0b11, 0b11, i) if (i & 0b1000) == 0b0000 || (i & 0b1100) == 0b1000 => {
                CsrType::StandardRO
            }
            (0b11, 0b11, i) if (i & 0b1000) == 0b0000 || (i & 0b1100) == 0b1100 => {
                CsrType::CustomRO
            }
            _ => CsrType::CustomRO,
        }
    }
}

impl Add for CsrAddress {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign for CsrAddress {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0
    }
}

impl Add<u16> for CsrAddress {
    type Output = Self;

    fn add(self, rhs: u16) -> Self::Output {
        Self(self.0 + (rhs & 0xFFF))
    }
}

impl AddAssign<u16> for CsrAddress {
    fn add_assign(&mut self, rhs: u16) {
        self.0 += (rhs & 0xFFF)
    }
}

impl Sub for CsrAddress {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl From<CsrAddress> for usize {
    fn from(val: CsrAddress) -> Self {
        val.0 as usize
    }
}

impl From<CsrAddress> for u16 {
    fn from(val: CsrAddress) -> Self {
        val.0
    }
}

impl From<u16> for CsrAddress {
    fn from(value: u16) -> Self {
        Self(value & 0xFFF)
    }
}

impl From<i64> for CsrAddress {
    fn from(value: i64) -> Self {
        Self(value as u16 & 0xFFF)
    }
}
