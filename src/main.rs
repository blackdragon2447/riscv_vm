use std::fs;

use riscv_vm::elf_load::elf_header::ElfHeader;

fn main() {
    let bytes = fs::read("./test_os/os.elf").unwrap();
    let header = ElfHeader::from_bytes(&bytes).unwrap();
    dbg!(header);
}
