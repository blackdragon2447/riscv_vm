use std::fs;

use elf_load::Elf;
#[cfg(feature = "vga_text_buf")]
use riscv_vm::devices::vga_text_mode::VgaTextMode;
use riscv_vm::{devices::simple_uart::SimpleUart, memory::KB, vmstate::VMStateBuilder};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let bytes = fs::read(&args[1]).unwrap();
    let elf = Elf::from_bytes(bytes).unwrap();

    let mut vmstate = VMStateBuilder::<{ 4 * KB }>::default().build();
    vmstate.load_elf_kernel(&elf).unwrap();

    vmstate
        .add_sync_device::<SimpleUart>(0x10000000u64.into())
        .unwrap();

    #[cfg(feature = "vga_text_buf")]
    vmstate
        .add_async_device::<VgaTextMode>(0xB8000u64.into())
        .unwrap();

    loop {
        // dbg!(&vmstate);
        vmstate.step().unwrap();
        // let mut buf = String::new();
        // stdin().read_line(&mut buf).unwrap();
    }
}
