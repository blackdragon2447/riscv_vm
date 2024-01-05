use std::fs;

use elf_load::Elf;
use riscv_vm::{memory::MB, vmstate::VMState};

fn main() {
    let bytes = fs::read("./test_os/os.elf").unwrap();
    let mut elf = Elf::from_bytes(bytes).unwrap();
    elf.populate_secion_names().unwrap();

    dbg!(&elf.header);
    dbg!(&elf.program_headers);
    dbg!(&elf.section_headers);

    let mut vmstate = VMState::<{ 4 * MB }>::new(1);
    vmstate.load_elf_kernel(&elf).unwrap();
    dbg!(&vmstate);

    vmstate.step().unwrap();
    vmstate.step().unwrap();
}
