pub struct PMP {
    pmpcfg: [PmpCfg; 64],
    pmpaddr: [u64; 64],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PmpCfg {
    read: bool,
    write: bool,
    exec: bool,
    addr_match: AddressMatch,
    locked: bool,
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
            if !cfg.locked {
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
            if !cfg.locked {
                *cfg = new;
            }
        }
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
        Self {
            read,
            write,
            exec,
            addr_match,
            locked,
        }
    }

    fn from_bits(bits: u8) -> Self {
        Self {
            read: (bits & 0b1) > 0,
            write: (bits & (0b1 << 1)) > 0,
            exec: (bits & (0b1 << 2)) > 0,
            addr_match: ((bits & (0b11 << 3)) >> 3).into(),
            locked: (bits & (0b1 << 7)) > 0,
        }
    }

    fn to_bits(&self) -> u8 {
        let mut bits = 0;

        bits |= self.read as u8;
        bits |= (self.write as u8) << 1;
        bits |= (self.exec as u8) << 2;
        bits |= (self.addr_match as u8) << 4;
        bits |= (self.locked as u8) << 7;

        bits
    }
}
