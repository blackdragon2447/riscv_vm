use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Sub},
};

use super::paging::AddressTranslationMode;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Address(u64);

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct VirtAddress {
    /// 3 and 4 may be 0 if not in use
    pub raw: u64,
    pub vpn: [u16; 5],
    pub page_offset: u16,
}

impl VirtAddress {
    pub fn from_address(addr: Address, mode: AddressTranslationMode) -> Self {
        let page_offset = (addr.0 & 0xFFF) as u16;
        let raw = addr.0;
        match mode {
            AddressTranslationMode::Bare => {
                unreachable!("We should never have virt addrs in bare mode")
            }
            AddressTranslationMode::Sv39 => {
                let mut vpn = [0; 5];
                vpn[0] = ((addr.0 >> 12) & 0x1FF) as u16;
                vpn[1] = ((addr.0 >> 21) & 0x1FF) as u16;
                vpn[2] = ((addr.0 >> 30) & 0x1FF) as u16;
                Self {
                    raw,
                    vpn,
                    page_offset,
                }
            }
            AddressTranslationMode::Sv48 => {
                let mut vpn = [0; 5];
                vpn[0] = ((addr.0 >> 12) & 0x1FF) as u16;
                vpn[1] = ((addr.0 >> 21) & 0x1FF) as u16;
                vpn[2] = ((addr.0 >> 30) & 0x1FF) as u16;
                vpn[3] = ((addr.0 >> 39) & 0x1FF) as u16;
                Self {
                    raw,
                    vpn,
                    page_offset,
                }
            }
            AddressTranslationMode::Sv57 => {
                let mut vpn = [0; 5];
                vpn[0] = ((addr.0 >> 12) & 0x1FF) as u16;
                vpn[1] = ((addr.0 >> 21) & 0x1FF) as u16;
                vpn[2] = ((addr.0 >> 30) & 0x1FF) as u16;
                vpn[3] = ((addr.0 >> 39) & 0x1FF) as u16;
                vpn[4] = ((addr.0 >> 48) & 0x1FF) as u16;
                Self {
                    raw,
                    vpn,
                    page_offset,
                }
            }
        }
    }
}

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
        Self(self.0.saturating_add_signed(rhs as i64))
    }
}

impl AddAssign<i32> for Address {
    fn add_assign(&mut self, rhs: i32) {
        self.0 = self.0.saturating_add_signed(rhs as i64);
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

impl From<Address> for i32 {
    fn from(val: Address) -> Self {
        val.0 as i32
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

impl From<i32> for Address {
    fn from(value: i32) -> Self {
        Self(value as u64)
    }
}

impl From<elf_load::Address> for Address {
    fn from(value: elf_load::Address) -> Self {
        Self(value.0)
    }
}
