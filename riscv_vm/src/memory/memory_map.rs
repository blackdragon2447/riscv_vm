use std::ops::Range;

use super::{address::Address, DeviceRegionId};

#[derive(Debug)]
#[allow(unused)]
pub enum MemoryRegion {
    Ram(Range<Address>),
    Rom(Range<Address>),
    IO(DeviceRegionId, Range<Address>),
}

impl MemoryRegion {
    pub(super) fn range(&self) -> Range<Address> {
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
    a.start < b.end && b.start < a.end
}

#[test]
fn overlap_test() {
    assert!(overlap(0..10, 2..8));
    assert!(overlap(2..8, 0..10));

    assert!(overlap(0..10, 8..12));
    assert!(overlap(8..12, 0..10));

    assert!(overlap(8..10, 0..10));
}
