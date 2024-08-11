// Because I want it!
#![allow(clippy::upper_case_acronyms)]

mod decode;
pub mod devices;
mod execute;
mod hart;
mod interrupt;
mod memory;

pub use crate::hart::trap;
pub use memory::{address::Address, KB, MB};

#[cfg(test)]
mod tests;
#[cfg(test)]
mod vm_tests;
pub mod vmstate;

#[rustfmt::skip]
pub const SUPPORTED_EXTENTIONS: &[&str] = &[
    "RV64I",
    "M",
    "Zmmul",  // Implied by M
    "A",      // Outdated? Handle rl/aq bits
    "Zalrsc", // Check if Reservations are up to spec, LR/SC loops
    "Zaamo",
    "F", 
    "D", 
    "C",
    // Privileged
    "S",
    "U",
    "Zicsr",
    "Zicntr",
    "Smcsrind/Sscsrind", // Want
    "RVWMO", // As long as we execute sync this should be guarenteed, once we go async this needs
             // to be manually verified.
];

pub const UNSUPPORTED_EXENTIONS: &[&str] = &[
    "Zifencei",
    "Zihpm",
    "Zihintntl",
    "Zihintpause",
    "Zimop",
    "Zcmop",
    "Zicond", // Want
    "Zawrs",
    "Zacas", // Want?
    "Ztso",
    "CMO",
    "Q",
    "Zfh",
    "Zfhmin",
    "Zfa",
    "Zfinx",
    "Zdinx",
    "Zhinx",
    "Zhinxmin",
    "B",
    "J",
    "P",
    // Privileged
    "Ssstaten/Smstaten",
    "Smepmp",
    "Smcntrpmf",
    "Smcdeleg",
    "Svnapot", // Bits exist but ro 0
    "Svpbmt",  // Bits exist but ro 0
    "Svinval",
    "Svadu",
    "Svvptc",
    "Sstc", // Want
    "Sscofpmf",
    "Smaia/Ssaia",
    "H",
];
