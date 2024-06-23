use std::time::{Duration, Instant};

use crate::hart::{
    self,
    trap::{Interrupt, InterruptTarget},
};

pub struct MTimer {
    base: Instant,
    time: Instant,
    time_cmp: Vec<Option<u64>>,
}

impl MTimer {
    pub fn new(hart_count: usize) -> Self {
        let base = Instant::now();
        Self {
            base,
            time: base,
            time_cmp: vec![None; hart_count],
        }
    }

    pub fn get_cmps(&self) -> &Vec<Option<u64>> {
        &self.time_cmp
    }

    pub fn get_time_micros(&self) -> u64 {
        self.time.elapsed().as_micros() as u64
    }

    pub fn set_time_micros(&mut self, micros: u64) {
        self.time = self.base - Duration::from_micros(micros);
    }

    pub fn get_cmp_micros(&self, hartid: u64) -> u64 {
        self.time_cmp
            .get(hartid as usize)
            .unwrap_or(&None)
            .unwrap_or(0)
    }

    pub fn set_cmp_micros(&mut self, micros: u64, hartid: u64) {
        if micros == 0 {
            self.time_cmp[hartid as usize] = None;
        } else {
            self.time_cmp[hartid as usize] = Some(micros);
        }

        if (micros as u128) > self.time.elapsed().as_micros() {
            // self.bus
            //     .clear_interrupt(InterruptTarget::Single(hartid as usize), Interrupt::Timer)
            //     .unwrap();
        }
    }

    pub fn generate_interrupts(&self) {
        // for (i, t) in self.time_cmp.iter().enumerate() {
        //     if t.is_some_and(|t| (t as u128) < self.time.elapsed().as_micros()) {
        //         bus.send_interrupt(InterruptTarget::Single(i), Interrupt::Timer)
        //             .unwrap();
        //     }
        // }
    }
}
