#![allow(dead_code)]
#![allow(unused)]

mod decode;
pub mod devices;
mod execute;
mod hart;
mod memory;

pub use crate::hart::trap;
pub use memory::{address::Address, KB, MB};

#[cfg(test)]
mod tests;
#[cfg(test)]
mod vm_tests;
pub mod vmstate;
