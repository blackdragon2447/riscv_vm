use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Sub},
};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Address(u64);

impl Debug for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#8x}", self.0)
    }
}

impl Add for Address {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign for Address {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0
    }
}

impl Add<u64> for Address {
    type Output = Self;

    fn add(self, rhs: u64) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl AddAssign<u64> for Address {
    fn add_assign(&mut self, rhs: u64) {
        self.0 += rhs
    }
}

impl Add<i32> for Address {
    type Output = Self;

    fn add(self, rhs: i32) -> Self::Output {
        Self(self.0.saturating_add_signed(rhs.into()))
    }
}

impl AddAssign<i32> for Address {
    fn add_assign(&mut self, rhs: i32) {
        self.0 = self.0.saturating_add_signed(rhs.into());
    }
}

impl Sub for Address {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl From<Address> for usize {
    fn from(val: Address) -> Self {
        val.0 as usize
    }
}

impl From<Address> for u64 {
    fn from(val: Address) -> Self {
        val.0
    }
}

impl From<Address> for i64 {
    fn from(val: Address) -> Self {
        val.0 as i64
    }
}

impl From<u64> for Address {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl From<i64> for Address {
    fn from(value: i64) -> Self {
        Self(value as u64)
    }
}

impl From<elf_load::Address> for Address {
    fn from(value: elf_load::Address) -> Self {
        Self(value.0)
    }
}
