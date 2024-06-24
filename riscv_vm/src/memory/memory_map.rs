use std::ops::{Range, RangeInclusive};

use super::{address::Address, DeviceRegionId, MemoryError};

#[derive(Debug)]
pub enum MemoryRegion {
    Ram(RangeInclusive<Address>),
    Rom(RangeInclusive<Address>),
    IO(DeviceRegionId, RangeInclusive<Address>),
}

impl MemoryRegion {
    pub(super) fn range(&self) -> RangeInclusive<Address> {
        match self {
            MemoryRegion::Ram(r) => r.clone(),
            MemoryRegion::Rom(r) => r.clone(),
            MemoryRegion::IO(_, r) => r.clone(),
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
    pub(super) fn new(ram: RangeInclusive<Address>) -> Self {
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

fn overlap<T: Ord>(a: RangeInclusive<T>, b: RangeInclusive<T>) -> bool {
    a.start() <= b.end() && b.start() <= a.end()
}
