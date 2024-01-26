macro_rules! isa_test {
    ($name:ident) => {
        #[test]
        fn $name() -> Result<(), u32> {
            let bytes = fs::read(format!(
                "../riscv-tests/isa/{}",
                stringify!($name).replace("_", "-")
            ))
            .unwrap();
            let elf = Elf::from_bytes(bytes).unwrap();

            let mut vmstate = VMState::new::<{ 6 * KB }>(1);
            vmstate.load_elf_kernel(&elf).unwrap();

            loop {
                vmstate.step();
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
    };
    ($name:ident, $mem:block) => {
        #[test]
        fn $name() -> Result<(), u32> {
            let bytes = fs::read(format!(
                "../riscv-tests/isa/{}",
                stringify!($name).replace("_", "-")
            ))
            .unwrap();
            let elf = Elf::from_bytes(bytes).unwrap();

            let mut vmstate = VMState::new::<$mem>(1);
            vmstate.load_elf_kernel(&elf).unwrap();

            loop {
                vmstate.step();
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
    };

    ($name:ident, $file:expr) => {
        #[test]
        fn $name() -> Result<(), u32> {
            let bytes = fs::read(format!("../riscv-tests/isa/{}", $file)).unwrap();
            let elf = Elf::from_bytes(bytes).unwrap();

            let mut vmstate = VMState::new::<{ 6 * KB }>(1);
            vmstate.load_elf_kernel(&elf).unwrap();

            loop {
                vmstate.step();
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
    };
    ($name:ident, $file:expr, $mem:block) => {
        #[test]
        fn $name() -> Result<(), u32> {
            let bytes = fs::read(format!("../riscv-tests/isa/{}", $file)).unwrap();
            let elf = Elf::from_bytes(bytes).unwrap();

            let mut vmstate = VMState::new::<$mem>(1);
            vmstate.load_elf_kernel(&elf).unwrap();

            loop {
                vmstate.step();
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
    };
}
