use std::fs;

use elf_load::Elf;
use riscv_vm::{devices::simple_uart::SimpleUart, memory::MB, vmstate::VMState};

fn main() {
    let bytes = fs::read("./test_os/os.elf").unwrap();
    let elf = Elf::from_bytes(bytes).unwrap();

    let mut vmstate = VMState::<{ 4 * MB }>::new(1);
    vmstate.load_elf_kernel(&elf).unwrap();

    vmstate
        .add_device::<SimpleUart>(0x10000000u64.into())
        .unwrap();

    loop {
        vmstate.step().unwrap();
        // dbg!(&vmstate);
        // let mut buf = String::new();
        // stdin().read_line(&mut buf).unwrap();
    }
}
