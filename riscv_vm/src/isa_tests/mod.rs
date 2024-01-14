mod util;

use std::{fs, process::exit};

use elf_load::Elf;

use crate::{memory::KB, vmstate::VMState};

#[test]
fn add() -> Result<(), u32> {
    let bytes = fs::read("../rv64ui-p-add").unwrap();
    let elf = Elf::from_bytes(bytes).unwrap();

    let mut vmstate = VMState::<{ 6 * KB }>::new(1);
    vmstate.load_elf_kernel(&elf).unwrap();

    loop {
        vmstate.step();
        dbg!(&vmstate);
        let bytes = u32::from_le_bytes(
            vmstate
                .mem()
                .read_bytes(0x80001000u64.into(), 4)
                .unwrap()
                .try_into()
                .unwrap(),
        );
        if (bytes & 0b1) == 1 {
            if (bytes >> 1) == 0 {
                return Ok(());
            } else {
                return Err(bytes >> 1);
            }
        }
    }
}
