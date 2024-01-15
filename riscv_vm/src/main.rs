use std::{fs, io::stdin};

use elf_load::Elf;
use riscv_vm::{
    devices::{simple_uart::SimpleUart, vga_text_mode::VgaTextMode},
    memory::MB,
    vmstate::VMState,
};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let bytes = fs::read(args.get(1).unwrap()).unwrap();
    let elf = Elf::from_bytes(bytes).unwrap();

    let mut vmstate = VMState::<{ 4 * MB }>::new(1);
    vmstate.load_elf_kernel(&elf).unwrap();

    // vmstate
    //     .add_sync_device::<SimpleUart>(0x10000000u64.into())
    //     .unwrap();

    // vmstate
    //     .add_async_device::<VgaTextMode>(0xB8000u64.into())
    //     .unwrap();

    loop {
        dbg!(&vmstate);
        vmstate.step().unwrap();
        let mut buf = String::new();
        stdin().read_line(&mut buf).unwrap();
    }
}
