use std::sync::{Arc, RwLock};

use crate::{
    devices::{handled_device::HandledDevice, Device, DeviceObject},
    memory::memory_buffer::{MemoryBuffer, NaiveBuffer},
};

#[derive(Debug)]
pub struct TestOutputDevice;

impl Device for TestOutputDevice {
    const MEM_SIZE: u64 = 128;

    fn new() -> Self {
        Self
    }
}

impl DeviceObject for TestOutputDevice {
    fn init(
        &mut self,
        mut mem: crate::devices::DeviceMemHandle,
    ) -> Result<(), crate::devices::DeviceInitError> {
        mem.add_memory_buffer(0x70000000u64.into(), NaiveBuffer::<128>::new());
        Ok(())
    }
}

impl HandledDevice for TestOutputDevice {
    fn update(&mut self) -> Result<(), crate::devices::DeviceError> {
        Ok(())
    }
}

macro_rules! isa_test {
    (off(tohost: $tohost:expr): $name:ident) => {
        #[test]
        fn $name() -> Result<(), u32> {
            let bytes = fs::read(format!(
                "../vm_tests/official_tests/isa/{}",
                stringify!($name).replace("_", "-")
            ))
            .unwrap();
            let elf = Elf::from_bytes(bytes).unwrap();

            let mut vmstate = VMStateBuilder::<{ (4 * KB) + 128 }>::default()
                .set_hart_count(1)
                .build()
                .unwrap();

            vmstate.load_elf_kernel(&elf).unwrap();

            loop {
                vmstate.step(false);
                let bytes = u32::from_le_bytes(
                    vmstate
                        .mem()
                        // .read_bytes(0x80001000u64.into(), 4)
                        .read_bytes($tohost.into(), 4)
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
    };
    (off(tohost: $tohost:expr): $name:ident, $mem:block) => {
        #[test]
        fn $name() -> Result<(), u32> {
            let bytes = fs::read(format!(
                "../vm_tests/official_tests/isa/{}",
                stringify!($name).replace("_", "-")
            ))
            .unwrap();
            let elf = Elf::from_bytes(bytes).unwrap();

            let mut vmstate = VMStateBuilder::<$mem>::default()
                .set_hart_count(1)
                .build()
                .unwrap();
            vmstate.load_elf_kernel(&elf).unwrap();

            loop {
                vmstate.step(false);
                let bytes = u32::from_le_bytes(
                    vmstate
                        .mem()
                        .read_bytes($tohost.into(), 4)
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
    };

    (off(tohost: $tohost:expr): $name:ident, $file:expr) => {
        #[test]
        fn $name() -> Result<(), u32> {
            let bytes = fs::read(format!("../vm_tests/official_tests/isa/{}", $file)).unwrap();
            let elf = Elf::from_bytes(bytes).unwrap();

            let mut vmstate = VMStateBuilder::<{ (4 * KB) + 128 }>::default()
                .set_hart_count(1)
                .build()
                .unwrap();
            vmstate.load_elf_kernel(&elf).unwrap();

            loop {
                vmstate.step(false);
                let bytes = u32::from_le_bytes(
                    vmstate
                        .mem()
                        .read_bytes($tohost.into(), 4)
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
    };
    (off(tohost: $tohost:expr): $name:ident, $file:expr, $mem:block) => {
        #[test]
        fn $name() -> Result<(), u32> {
            let bytes = fs::read(format!("../vm_tests/official_tests/isa/{}", $file)).unwrap();
            let elf = Elf::from_bytes(bytes).unwrap();

            let mut vmstate = VMStateBuilder::<$mem>::default()
                .set_hart_count(1)
                .build()
                .unwrap();
            vmstate.load_elf_kernel(&elf).unwrap();

            loop {
                vmstate.step(false);
                let bytes = u32::from_le_bytes(
                    vmstate
                        .mem()
                        .read_bytes($tohost.into(), 4)
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
    };

    (custom: $name:ident) => {
        #[test]
        fn $name() -> Result<(), u32> {
            let bytes = fs::read(format!(
                "../vm_tests/custom_tests/out/{}",
                stringify!($name).replace("_", "-")
            ))
            .unwrap();
            let elf = Elf::from_bytes(bytes).unwrap();

            let mut vmstate = VMStateBuilder::<{ (4 * KB) + 128 }>::default()
                .set_hart_count(1)
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
    };
    (custom: $name:ident, $mem:block) => {
        #[test]
        fn $name() -> Result<(), u32> {
            let bytes = fs::read(format!(
                "../vm_tests/custom_tests/out/{}",
                stringify!($name).replace("_", "-")
            ))
            .unwrap();
            let elf = Elf::from_bytes(bytes).unwrap();

            let mut vmstate = VMStateBuilder::<$mem>::default()
                .set_hart_count(1)
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
    };

    (custom: $name:ident, $file:expr) => {
        #[test]
        fn $name() -> Result<(), u32> {
            let bytes = fs::read(format!("../vm_tests/custom_tests/out/{}", $file)).unwrap();
            let elf = Elf::from_bytes(bytes).unwrap();

            let mut vmstate = VMStateBuilder::<{ (4 * KB) + 128 }>::default()
                .set_hart_count(1)
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
    };
    (custom: $name:ident, $file:expr, $mem:block) => {
        #[test]
        fn $name() -> Result<(), u32> {
            let bytes = fs::read(format!("../vm_tests/custom_tests/out/{}", $file)).unwrap();
            let elf = Elf::from_bytes(bytes).unwrap();

            let mut vmstate = VMStateBuilder::<$mem>::default()
                .set_hart_count(1)
                .add_sync_device::<TestOutputDevice>(0x70000000u64.into())
                .build()
                .unwrap();

            vmstate.load_elf_kernel(&elf).unwrap();

            loop {
                vmstate.step();
                let bytes = u32::from_le_bytes(
                    vmstate
                        .mem()
                        .read_bytes(0x70000000u64.into(), 4, PrivilegeMode::Machine, None)
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
    };
}
