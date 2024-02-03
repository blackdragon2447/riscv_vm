use std::{
    fs::{self, File},
    io::Write,
    process::exit,
};

use elf_load::Elf;

use crate::{
    hart::privilege::PrivilegeMode,
    memory::pmp::PMP,
    memory::{KB, MB},
    vm_tests::util::TestOutputDevice,
    vmstate::VMState,
    vmstate::VMStateBuilder,
};

isa_test!(custom: rv64si_v_paging, {1 * MB});
isa_test!(custom: rv64ui_p_pass);
