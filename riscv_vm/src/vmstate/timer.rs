use std::{
    rc::Rc,
    sync::Mutex,
    time::{Duration, Instant},
};

use enumflags2::BitFlags;
use nohash_hasher::IntMap;

use crate::{
    hart::{
        self,
        trap::{Interrupt, InterruptTarget},
    },
    memory::memory_buffer::{MemoryBuffer, MemoryBufferError},
    trap::InterruptInternal,
    Address,
};

pub struct TimerRef(Rc<Mutex<Instant>>);

impl TimerRef {
    pub fn get_time(&self) -> u64 {
        self.0.lock().unwrap().elapsed().as_micros() as u64
    }

    #[cfg(test)]
    pub(crate) fn dummy() -> Self {
        Self(Rc::new(Mutex::new(Instant::now())))
    }
}

pub struct MTimer {
    time: Rc<Mutex<Instant>>,
    time_cmp: Vec<Option<u64>>,
    interrupts: IntMap<usize, Rc<Mutex<BitFlags<InterruptInternal>>>>,
    hart_count: usize,
}

impl MemoryBuffer for MTimer {
    fn size(&self) -> u64 {
        (self.hart_count as u64 + 1) * 8
    }

    fn write_bytes(&mut self, bytes: &[u8], addr: Address) -> Result<(), MemoryBufferError> {
        let addr = <Address as Into<u64>>::into(addr);
        if addr % 8 != 0 {
            return Err(MemoryBufferError::UnalignedWrite(addr.into()));
        }
        let mut num_bytes = [0u8; 8];
        num_bytes[0..bytes.len()].copy_from_slice(bytes);
        let micros = u64::from_le_bytes(num_bytes);
        if addr == 0 {
            self.set_time_micros(micros);
        } else {
            self.set_cmp_micros(micros, (addr / 8) - 1);
        }
        Ok(())
    }

    fn read_bytes(&self, addr: Address, size: usize) -> Result<Vec<u8>, MemoryBufferError> {
        let addr = <Address as Into<u64>>::into(addr);
        if addr % 8 != 0 {
            return Err(MemoryBufferError::UnalignedRead(addr.into()));
        }
        if addr == 0 {
            Ok(self.get_time_micros().to_le_bytes()[0..size].to_vec())
        } else {
            Ok(self.get_cmp_micros((addr / 8) - 1).to_le_bytes()[0..size].to_vec())
        }
    }
}

impl MTimer {
    pub fn new(hart_count: usize) -> Self {
        Self {
            time: Rc::new(Mutex::new(Instant::now())),
            time_cmp: vec![None; hart_count],
            interrupts: IntMap::default(),
            hart_count,
        }
    }

    pub fn get_ref(&self) -> TimerRef {
        TimerRef(self.time.clone())
    }

    pub fn get_cmps(&self) -> &Vec<Option<u64>> {
        &self.time_cmp
    }

    pub fn get_time_micros(&self) -> u64 {
        self.time.lock().unwrap().elapsed().as_micros() as u64
    }

    pub fn set_time_micros(&mut self, micros: u64) {
        *self.time.lock().unwrap() = Instant::now() - Duration::from_micros(micros);
    }

    pub fn get_cmp_micros(&self, hartid: u64) -> u64 {
        self.time_cmp
            .get(hartid as usize)
            .unwrap_or(&None)
            .unwrap_or(0)
    }

    pub fn set_cmp_micros(&mut self, micros: u64, hartid: u64) {
        eprintln!("Set cmp to {micros} for H{hartid}");
        if micros == 0 {
            self.time_cmp[hartid as usize] = None;
        } else {
            self.time_cmp[hartid as usize] = Some(micros);
        }

        if (micros as u128) < self.time.lock().unwrap().elapsed().as_micros() {
            self.interrupts
                .get(&(hartid as usize))
                .as_mut()
                .map(|bits| *bits.lock().unwrap() |= InterruptInternal::MachineTimer);
        } else {
            self.interrupts
                .get(&(hartid as usize))
                .as_mut()
                .map(|bits| *bits.lock().unwrap() &= !InterruptInternal::MachineTimer);
        }
    }

    pub fn generate_interrupts(&self) {
        for (i, t) in self.time_cmp.iter().enumerate() {
            if t.is_some_and(|t| (t as u128) < self.time.lock().unwrap().elapsed().as_micros()) {
                self.interrupts
                    .get(&i)
                    .as_mut()
                    .map(|bits| *bits.lock().unwrap() |= InterruptInternal::MachineTimer);
            }
        }
    }

    pub fn add_interrupt_bits(
        &mut self,
        hartid: usize,
        bits: Rc<Mutex<BitFlags<InterruptInternal>>>,
    ) {
        self.interrupts.insert(hartid, bits);
    }
}
