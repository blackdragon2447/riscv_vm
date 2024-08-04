use std::{
    fs::{self, File},
    io::Write,
    process::exit,
};

use elf_load::Elf;

use crate::{
    hart::privilege::PrivilegeMode,
    memory::{pmp::PMP, KB, MB},
    vm_tests::util::TestOutputDevice,
    vmstate::{VMSettings, VMState, VMStateBuilder},
};

isa_test!(custom: rv64si_v_paging, {1 * MB});
#[test]
fn rv64ui_v_software_interrupt() -> Result<(), u32> {
    let bytes = fs::read(format!(
        "../vm_tests/custom_tests/out/rv64si-v-software_interrupt"
    ))
    .unwrap();
    let elf = Elf::from_bytes(bytes).unwrap();

    let mut vmstate = VMStateBuilder::<{ (4 * KB) + 128 }>::new(VMSettings {
        m_mode_swi_enable: true,
        ..Default::default()
    })
    .set_hart_count(2)
    .add_sync_device::<TestOutputDevice>(0x70000000u64.into())
    .build()
    .unwrap();

    vmstate.load_elf_kernel(&elf).unwrap();

    loop {
        vmstate.step(false);
        let bytes = u32::from_le_bytes(
            vmstate
                .mem()
                .read_bytes(0x70000000u64.into(), 4)
                .unwrap()
                .try_into()
                .unwrap(),
        );
        if (bytes & 0b1) == 1 {
            if (bytes >> 1) == 0 {
                return Ok(());
            }
            return Err(bytes >> 1);
        }
    }
}
isa_test!(custom: rv64ui_p_pass);
