use std::{fs, process::exit};

use elf_load::Elf;

use crate::{
    hart::privilege::PrivilegeMode,
    memory::pmp::PMP,
    memory::{KB, MB},
    vmstate::VMState,
    vmstate::VMStateBuilder,
};
