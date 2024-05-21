use std::ops::Range;

use crate::devices::DeviceId;

use super::{address::Address, registers::RegisterLength, MemoryError};

#[derive(Debug)]
pub enum MemoryRegion {
    Ram(Range<Address>),
    Rom(Range<Address>),
    IO(DeviceId, Range<Address>),
    Register(DeviceId, RegisterLength, Address),
}

impl MemoryRegion {
    pub(super) fn range(&self) -> Range<Address> {
        match self {
            MemoryRegion::Ram(r) => r.clone(),
            MemoryRegion::Rom(r) => r.clone(),
            MemoryRegion::IO(_, r) => r.clone(),
            // + 9 and not 8 because .. is
            // exclusivem to +8 would make our range only 7 wide.
            MemoryRegion::Register(_, l, a) => *a..(*a + (*l as u64) + 1u64),
        }
    }
}

#[derive(Debug)]
pub struct MemoryMap(Vec<MemoryRegion>);

#[derive(Debug)]
pub enum MemoryMapError {
    OutOfBounds,
    TooLarge,
    MultipleRamRegions,
    RegionOverlap,
}

impl MemoryMap {
    pub(super) fn new(ram: Range<Address>) -> Self {
        Self(vec![MemoryRegion::Ram(ram)])
    }

    pub(super) fn find(&self, addr: Address) -> Option<&MemoryRegion> {
        self.0.iter().find(|r| r.range().contains(&addr))
    }

    pub(super) fn fit(&self, range: Range<Address>) -> Result<&MemoryRegion, MemoryMapError> {
        if let Some(r) = self.find(range.start) {
            if r.range().contains(&range.end) {
                Ok(r)
            } else {
                Err(MemoryMapError::TooLarge)
            }
        } else {
            Err(MemoryMapError::OutOfBounds)
        }
    }

    pub(super) fn add_region(&mut self, region: MemoryRegion) -> Result<(), MemoryMapError> {
        if let MemoryRegion::Ram(_) = region {
            return Err(MemoryMapError::MultipleRamRegions);
        }
        if self.0.iter().any(|a| overlap(a.range(), region.range())) {
            Err(MemoryMapError::RegionOverlap)
        } else {
            self.0.push(region);
            Ok(())
        }
    }
}

fn overlap<T: Ord>(a: Range<T>, b: Range<T>) -> bool {
    a.start <= b.end && b.start <= a.end
}
