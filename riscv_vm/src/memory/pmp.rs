use std::ops::Range;

use enumflags2::{bitflags, BitFlags};

use super::address::Address;
use crate::hart::privilege::{self, PrivilegeMode};

#[derive(Debug)]
pub struct PMP {
    pmpcfg: [PmpCfg; 64],
    pmpaddr: [u64; 64],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PmpCfg {
    rwx: BitFlags<AccessMode>,
    addr_match: AddressMatch,
    pub locked: bool,
}

#[repr(u8)]
#[bitflags]
#[derive(Clone, Copy, Debug)]
pub enum AccessMode {
    Read = 0b1 << 0,
    Write = 0b1 << 1,
    Exec = 0b1 << 2,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AddressMatch {
    OFF = 0,
    TOR = 1,
    NA4 = 2,
    NAPOT = 3,
}

impl From<u8> for AddressMatch {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::OFF,
            1 => Self::TOR,
            2 => Self::NA4,
            3 => Self::NAPOT,
            _ => unreachable!(),
        }
    }
}

impl PMP {
    pub fn new() -> Self {
        Self {
            pmpcfg: [PmpCfg::from_bits(0); 64],
            pmpaddr: [0; 64],
        }
    }

    pub fn read_cfg_rv32(&self, idx: usize) -> u32 {
        u32::from_le_bytes(
            self.pmpcfg[(idx * 4)..((idx + 1) * 4)]
                .iter()
                .map(|p| p.to_bits())
                .collect::<Vec<u8>>()
                .try_into()
                .unwrap(),
        )
    }

    pub fn write_cfg_rv32(&mut self, idx: usize, value: u32) {
        let low = idx * 4;
        let values = value.to_le_bytes().map(|b| PmpCfg::from_bits(b));
        for (cfg, new) in (&mut self.pmpcfg[low..(low + 4)]).iter_mut().zip(values) {
            if !(cfg.locked
                || (new.rwx.contains(AccessMode::Write) && !new.rwx.contains(AccessMode::Read)))
            {
                *cfg = new;
            }
        }
    }

    pub fn read_cfg_rv64(&self, idx: usize) -> u64 {
        if idx % 2 != 0 {
            panic!("Index of 64bit pmpcfg must be even");
        }
        u64::from_le_bytes(
            self.pmpcfg[(idx * 4)..((idx + 2) * 4)]
                .iter()
                .map(|p| p.to_bits())
                .collect::<Vec<u8>>()
                .try_into()
                .unwrap(),
        )
    }

    pub fn write_cfg_rv64(&mut self, idx: usize, value: u64) {
        if idx % 2 != 0 {
            panic!("Index of 64bit pmpcfg must be even");
        }
        let low = idx * 4;
        let values = value.to_le_bytes().map(|b| PmpCfg::from_bits(b));
        for (cfg, new) in (&mut self.pmpcfg[low..(low + 8)]).iter_mut().zip(values) {
            if !(cfg.locked
                || (new.rwx.contains(AccessMode::Write) && !new.rwx.contains(AccessMode::Read)))
            {
                *cfg = new;
            }
        }
    }

    pub fn ranges(&self) -> Vec<(&PmpCfg, Range<Address>)> {
        let mut result = Vec::with_capacity(64);
        for (i, cfg) in self.pmpcfg.iter().enumerate() {
            match cfg.addr_match {
                AddressMatch::OFF => (),
                AddressMatch::TOR if i == 0 => {
                    result.push((cfg, ((0u64.into())..(self.pmpaddr[i] << 2).into())))
                }
                AddressMatch::TOR if i > 0 => result.push((
                    cfg,
                    ((self.pmpaddr[i - 1] << 2).into()..(self.pmpaddr[i] << 2).into()),
                )),
                AddressMatch::NA4 => result.push((
                    cfg,
                    ((self.pmpaddr[i] << 2).into()..((self.pmpaddr[i] << 2) + 4).into()),
                )),

                AddressMatch::NAPOT => {
                    let mut addr = self.pmpaddr[i];
                    let mut size = 3;
                    while addr % 2 != 0 {
                        size += 1;
                        addr >>= 1;
                    }
                    let mask = -1i64 as u64; // All bits set
                    let low_mask = mask << size; // clean bottom size bits
                    let high_mask = !low_mask; // set bottom size bits;
                    let low = ((self.pmpaddr[i] << 2) & low_mask).into();
                    let high = ((self.pmpaddr[i] << 2) | high_mask).into();
                    result.push((cfg, (low..high)));
                }
                _ => unreachable!(),
            }
        }

        result
    }

    pub fn check(&self, addr: Address, privilege: PrivilegeMode, mode: AccessMode) -> bool {
        if privilege < PrivilegeMode::Machine {
            for (p, r) in self.ranges() {
                if r.contains(&addr) {
                    return p.rwx.contains(mode);
                }
            }
            return false;
        } else {
            for (p, r) in self.ranges() {
                if p.locked {
                    if r.contains(&addr) {
                        return p.rwx.contains(mode);
                    }
                }
            }
            return true;
        }
    }

    pub fn read_addr_rv32(&self, idx: usize) -> u32 {
        self.pmpaddr[idx] as u32
    }

    pub fn write_addr_rv32(&mut self, idx: usize, addr: u32) {
        self.pmpaddr[idx] = addr as u64;
    }

    pub fn read_addr_rv64(&self, idx: usize) -> u64 {
        self.pmpaddr[idx] as u64
    }

    pub fn write_addr_rv64(&mut self, idx: usize, addr: u64) {
        self.pmpaddr[idx] = addr & 0x3FFFFFFFFFFFFF; // top 10 bits are WARL 0
    }

    pub fn get_cfgs(&self) -> &[PmpCfg; 64] {
        &self.pmpcfg
    }
}

impl PmpCfg {
    #[cfg(test)]
    pub(in crate::memory) fn new_configured(
        read: bool,
        write: bool,
        exec: bool,
        addr_match: AddressMatch,
        locked: bool,
    ) -> Self {
        use enumflags2::BitFlag;

        let mut rwx = AccessMode::empty();
        if read {
            rwx |= AccessMode::Read
        }
        if write {
            rwx |= AccessMode::Write
        }
        if exec {
            rwx |= AccessMode::Exec
        }
        Self {
            rwx,
            addr_match,
            locked,
        }
    }

    fn from_bits(bits: u8) -> Self {
        Self {
            rwx: BitFlags::from_bits(bits & 0b111).unwrap(),
            addr_match: ((bits & (0b11 << 3)) >> 3).into(),
            locked: (bits & (0b1 << 7)) > 0,
        }
    }

    pub fn to_bits(&self) -> u8 {
        let mut bits = 0;

        bits |= self.rwx.bits();
        bits |= (self.addr_match as u8) << 3;
        bits |= (self.locked as u8) << 7;

        bits
    }
}
