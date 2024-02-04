use crate::devices::{Device, HandledDevice};

pub struct TestOutputDevice;

impl Device for TestOutputDevice {
    const MEN_SIZE: u64 = 128;

    fn init(mem: &mut crate::memory::DeviceMemory) -> Result<Self, crate::devices::DeviceInitError>
    where
        Self: Sized,
    {
        Ok(Self)
    }
}

impl HandledDevice for TestOutputDevice {
    fn update(
        &mut self,
        mem: &mut crate::memory::DeviceMemory,
    ) -> Result<(), crate::devices::DeviceError> {
        Ok(())
    }
}

macro_rules! isa_test {
    (off: $name:ident) => {
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
                .build();

            vmstate.load_elf_kernel(&elf).unwrap();

            loop {
                vmstate.step(false);
                let bytes = u32::from_le_bytes(
                    vmstate
                        .mem()
                        .read_bytes(0x80001000u64.into(), 4, PrivilegeMode::Machine, None)
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
    };
    (off: $name:ident, $mem:block) => {
        #[test]
        fn $name() -> Result<(), u32> {
            let bytes = fs::read(format!(
                "../vm_tests/official_tests/isa/{}",
                stringify!($name).replace("_", "-")
            ))
            .unwrap();
            let elf = Elf::from_bytes(bytes).unwrap();

            let mut vmstate = VMStateBuilder::<$mem>::default().set_hart_count(1).build();
            vmstate.load_elf_kernel(&elf).unwrap();

            loop {
                vmstate.step(false);
                let bytes = u32::from_le_bytes(
                    vmstate
                        .mem()
                        .read_bytes(0x80001000u64.into(), 4, PrivilegeMode::Machine, None)
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
    };

    (off: $name:ident, $file:expr) => {
        #[test]
        fn $name() -> Result<(), u32> {
            let bytes = fs::read(format!("../vm_tests/official_tests/isa/{}", $file)).unwrap();
            let elf = Elf::from_bytes(bytes).unwrap();

            let mut vmstate = VMStateBuilder::<{ (4 * KB) + 128 }>::default()
                .set_hart_count(1)
                .build();
            vmstate.load_elf_kernel(&elf).unwrap();

            loop {
                vmstate.step();
                let bytes = u32::from_le_bytes(
                    vmstate
                        .mem()
                        .read_bytes(0x80001000u64.into(), 4, PrivilegeMode::Machine, None)
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
    };
    (off: $name:ident, $file:expr, $mem:block) => {
        #[test]
        fn $name() -> Result<(), u32> {
            let bytes = fs::read(format!("../vm_tests/official_tests/isa/{}", $file)).unwrap();
            let elf = Elf::from_bytes(bytes).unwrap();

            let mut vmstate = VMStateBuilder::<$mem>::default().set_hart_count(1).build();
            vmstate.load_elf_kernel(&elf).unwrap();

            loop {
                vmstate.step();
                let bytes = u32::from_le_bytes(
                    vmstate
                        .mem()
                        .read_bytes(0x80001000u64.into(), 4, PrivilegeMode::Machine, None)
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
    };

    (custom: $name:ident) => {
        #[test]
        fn $name() -> Result<(), u32> {
            let bytes = fs::read(format!(
                "../vm_tests/custom_tests/{}",
                stringify!($name).replace("_", "-")
            ))
            .unwrap();
            let elf = Elf::from_bytes(bytes).unwrap();

            let mut vmstate = VMStateBuilder::<{ (4 * KB) + 128 }>::default()
                .set_hart_count(1)
                .build();
            vmstate
                .add_sync_device::<TestOutputDevice>(0x70000000u64.into())
                .unwrap();

            vmstate.load_elf_kernel(&elf).unwrap();

            loop {
                vmstate.step(false);
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
                    } else {
                        return Err(bytes >> 1);
                    }
                }
            }
        }
    };
    (custom: $name:ident, $mem:block) => {
        #[test]
        fn $name() -> Result<(), u32> {
            let bytes = fs::read(format!(
                "../vm_tests/custom_tests/{}",
                stringify!($name).replace("_", "-")
            ))
            .unwrap();
            let elf = Elf::from_bytes(bytes).unwrap();

            let mut vmstate = VMStateBuilder::<$mem>::default().set_hart_count(1).build();
            vmstate
                .add_sync_device::<TestOutputDevice>(0x70000000u64.into())
                .unwrap();

            vmstate.load_elf_kernel(&elf).unwrap();

            loop {
                vmstate.step(false);
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
                    } else {
                        return Err(bytes >> 1);
                    }
                }
            }
        }
    };

    (custom: $name:ident, $file:expr) => {
        #[test]
        fn $name() -> Result<(), u32> {
            let bytes = fs::read(format!("../vm_tests/custom_tests/{}", $file)).unwrap();
            let elf = Elf::from_bytes(bytes).unwrap();

            let mut vmstate = VMStateBuilder::<{ (4 * KB) + 128 }>::default()
                .set_hart_count(1)
                .build();
            vmstate
                .add_sync_device::<TestOutputDevice>(0x70000000u64.into())
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
                    } else {
                        return Err(bytes >> 1);
                    }
                }
            }
        }
    };
    (custom: $name:ident, $file:expr, $mem:block) => {
        #[test]
        fn $name() -> Result<(), u32> {
            let bytes = fs::read(format!("../vm_tests/custom_tests/{}", $file)).unwrap();
            let elf = Elf::from_bytes(bytes).unwrap();

            let mut vmstate = VMStateBuilder::<$mem>::default().set_hart_count(1).build();
            vmstate
                .add_sync_device::<TestOutputDevice>(0x70000000u64.into())
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
                    } else {
                        return Err(bytes >> 1);
                    }
                }
            }
        }
    };
}
