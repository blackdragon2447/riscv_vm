use crate::{
    hart::privilege::PrivilegeMode,
    memory::{
        pmp::{AddressMatch, PmpCfg, PMP},
        MemoryError,
    },
};

use super::Memory;

#[cfg(test)]
mod pmp {

    use crate::memory::pmp::{AddressMatch, PmpCfg, PMP};

    #[test]
    fn rv32_pmp_read() {
        let pmp = PMP::default();
        assert_eq!(pmp.read_cfg_rv32(0), 0);
        assert_eq!(pmp.read_cfg_rv32(1), 0);
        assert_eq!(pmp.read_cfg_rv32(2), 0);
        assert_eq!(pmp.read_cfg_rv32(3), 0);
        assert_eq!(pmp.read_cfg_rv32(4), 0);
        assert_eq!(pmp.read_cfg_rv32(5), 0);
        assert_eq!(pmp.read_cfg_rv32(6), 0);
        assert_eq!(pmp.read_cfg_rv32(7), 0);
        assert_eq!(pmp.read_cfg_rv32(8), 0);
        assert_eq!(pmp.read_cfg_rv32(9), 0);
        assert_eq!(pmp.read_cfg_rv32(10), 0);
        assert_eq!(pmp.read_cfg_rv32(11), 0);
        assert_eq!(pmp.read_cfg_rv32(12), 0);
        assert_eq!(pmp.read_cfg_rv32(13), 0);
        assert_eq!(pmp.read_cfg_rv32(14), 0);
        assert_eq!(pmp.read_cfg_rv32(15), 0);
    }

    #[test]
    #[should_panic(expected = "range end index 68 out of range for slice of length 64")]
    fn rv32_pmp_read_oob() {
        let pmp = PMP::default();
        assert_eq!(pmp.read_cfg_rv32(16), 0);
    }

    #[test]
    fn rv64_pmp_read() {
        let pmp = PMP::default();
        assert_eq!(pmp.read_cfg_rv64(0), 0);
        assert_eq!(pmp.read_cfg_rv64(2), 0);
        assert_eq!(pmp.read_cfg_rv64(4), 0);
        assert_eq!(pmp.read_cfg_rv64(6), 0);
        assert_eq!(pmp.read_cfg_rv64(8), 0);
        assert_eq!(pmp.read_cfg_rv64(10), 0);
        assert_eq!(pmp.read_cfg_rv64(12), 0);
        assert_eq!(pmp.read_cfg_rv64(14), 0);
    }

    #[test]
    #[should_panic(expected = "range end index 72 out of range for slice of length 64")]
    fn rv64_pmp_read_oob() {
        let pmp = PMP::default();
        assert_eq!(pmp.read_cfg_rv64(16), 0);
    }

    #[test]
    #[should_panic(expected = "Index of 64bit pmpcfg must be even")]
    fn rv64_pmp_read_uneven() {
        let pmp = PMP::default();
        assert_eq!(pmp.read_cfg_rv64(1), 0);
    }

    #[test]
    fn rv32_write() {
        let mut pmp = PMP::default();
        pmp.write_cfg_rv32(0, 0b00001111_10001101_00001011_10001001);
        let cfgs = pmp.get_cfgs();
        assert_eq!(
            cfgs[3],
            PmpCfg::new_configured(true, true, true, AddressMatch::TOR, false)
        );
        assert_eq!(
            cfgs[2],
            PmpCfg::new_configured(true, false, true, AddressMatch::TOR, true)
        );
        assert_eq!(
            cfgs[1],
            PmpCfg::new_configured(true, true, false, AddressMatch::TOR, false)
        );
        assert_eq!(
            cfgs[0],
            PmpCfg::new_configured(true, false, false, AddressMatch::TOR, true)
        );
    }

    #[test]
    fn rv64_write() {
        let mut pmp = PMP::default();
        pmp.write_cfg_rv64(
            0,
            0b00001111_10001101_00001011_10001001_00011111_10011101_00011011_10011001,
        );
        let cfgs = pmp.get_cfgs();
        assert_eq!(
            cfgs[7],
            PmpCfg::new_configured(true, true, true, AddressMatch::TOR, false)
        );
        assert_eq!(
            cfgs[6],
            PmpCfg::new_configured(true, false, true, AddressMatch::TOR, true)
        );
        assert_eq!(
            cfgs[5],
            PmpCfg::new_configured(true, true, false, AddressMatch::TOR, false)
        );
        assert_eq!(
            cfgs[4],
            PmpCfg::new_configured(true, false, false, AddressMatch::TOR, true)
        );
        assert_eq!(
            cfgs[3],
            PmpCfg::new_configured(true, true, true, AddressMatch::NAPOT, false)
        );
        assert_eq!(
            cfgs[2],
            PmpCfg::new_configured(true, false, true, AddressMatch::NAPOT, true)
        );
        assert_eq!(
            cfgs[1],
            PmpCfg::new_configured(true, true, false, AddressMatch::NAPOT, false)
        );
        assert_eq!(
            cfgs[0],
            PmpCfg::new_configured(true, false, false, AddressMatch::NAPOT, true)
        );
    }

    #[test]
    fn rv64_ranges() {
        let mut pmp = PMP::default();
        pmp.write_cfg_rv64(
            0,
            0b00001111_10001101_00001011_10001001_00011111_10011101_00001011_10001001,
        );
        pmp.write_addr_rv64(2, (0x80000000 >> 2) | 0b011);
        pmp.write_addr_rv64(1, (0xB1FA0 >> 2));
        pmp.write_addr_rv64(0, (0xB1000 >> 2));
        let ranges = &pmp.ranges()[0..8];
        assert_eq!(
            ranges[2],
            (
                &PmpCfg::new_configured(true, false, true, AddressMatch::NAPOT, true),
                (0x80000000u64.into()..=0x8000001fu64.into())
            )
        );
        assert_eq!(
            ranges[1],
            (
                &PmpCfg::new_configured(true, true, false, AddressMatch::TOR, false),
                (0xB1000u64.into()..=0xB1FA0u64.into())
            )
        );
        assert_eq!(
            ranges[0],
            (
                &PmpCfg::new_configured(true, false, false, AddressMatch::TOR, true),
                (0x0u64.into()..=0xB1000u64.into())
            )
        );
    }
}

#[cfg(test)]
mod paging {
    use enumflags2::{make_bitflags, BitFlag};

    use crate::memory::paging::{AddressTranslationMode, Pte, PteFlags, PteType};

    #[test]
    fn leafs_sv39() {
        let pte0 = 0x0026_8149_7679_E847u64;
        let pte1 = 0x4036_11F6_CCF0_3CF7u64;
        let pte2 = 0xC010_95DA_756F_0855u64;
        let pte3 = 0xC008_B229_0A31_B8C9u64;
        let pte4 = 0x0013_BDA3_CD92_6867u64;
        let pte5 = 0x4018_2528_8D16_1434u64;
        let pte6 = 0x0009_DE02_DFDC_D48Cu64;
        let pte7 = 0xC020_CE55_6717_A445u64;

        let pte0 = PteType::from_bytes(pte0, AddressTranslationMode::Sv39);
        let pte1 = PteType::from_bytes(pte1, AddressTranslationMode::Sv39);
        let pte2 = PteType::from_bytes(pte2, AddressTranslationMode::Sv39);
        let pte3 = PteType::from_bytes(pte3, AddressTranslationMode::Sv39);
        let pte4 = PteType::from_bytes(pte4, AddressTranslationMode::Sv39);
        let pte5 = PteType::from_bytes(pte5, AddressTranslationMode::Sv39);
        let pte6 = PteType::from_bytes(pte6, AddressTranslationMode::Sv39);
        let pte7 = PteType::from_bytes(pte7, AddressTranslationMode::Sv39);

        let expected_pte0 = PteType::Leaf(Pte::Sv39 {
            flags: make_bitflags!(PteFlags::{ V | R | W | A }),
            ppn: [122, 207, 40375447],
            pbmt: 0,
            n: false,
        });

        let expected_pte1 = PteType::Leaf(Pte::Sv39 {
            flags: make_bitflags!(PteFlags::{ V | R | W | U | G | A | D }),
            ppn: [15, 414, 56696684],
            pbmt: 0,
            n: false,
        });

        let expected_pte2 = PteType::Leaf(Pte::Sv39 {
            flags: make_bitflags!(PteFlags::{ V | W | U | A }),
            ppn: [450, 173, 17391015],
            pbmt: 0,
            n: false,
        });

        let expected_pte3 = PteType::Leaf(Pte::Sv39 {
            flags: make_bitflags!(PteFlags::{ V | X | A | D }),
            ppn: [110, 326, 9118352],
            pbmt: 0,
            n: false,
        });

        let expected_pte4 = PteType::Leaf(Pte::Sv39 {
            flags: make_bitflags!(PteFlags::{ V | R | W | G | A }),
            ppn: [154, 434, 20699708],
            pbmt: 0,
            n: false,
        });

        let expected_pte5 = PteType::Leaf(Pte::Sv39 {
            flags: make_bitflags!(PteFlags::{ W | U | G }),
            ppn: [389, 418, 25318024],
            pbmt: 0,
            n: false,
        });

        let expected_pte6 = PteType::Leaf(Pte::Sv39 {
            flags: make_bitflags!(PteFlags::{ D | X | W }),
            ppn: [309, 507, 10346541],
            pbmt: 0,
            n: false,
        });

        let expected_pte7 = PteType::Leaf(Pte::Sv39 {
            flags: make_bitflags!(PteFlags::{ V | W | A }),
            ppn: [489, 226, 34399574],
            pbmt: 0,
            n: false,
        });

        assert_eq!(pte0, expected_pte0);
        assert_eq!(pte1, expected_pte1);
        assert_eq!(pte2, expected_pte2);
        assert_eq!(pte3, expected_pte3);
        assert_eq!(pte4, expected_pte4);
        assert_eq!(pte5, expected_pte5);
        assert_eq!(pte6, expected_pte6);
        assert_eq!(pte7, expected_pte7);
    }

    #[test]
    fn branches_sv39() {
        let pte0 = 0x0026_8149_7679_E801u64;
        let pte1 = 0x4036_11F6_CCF0_3C01u64;
        let pte2 = 0xC010_95DA_756F_0801u64;
        let pte3 = 0xC008_B229_0A31_B801u64;
        let pte4 = 0x0013_BDA3_CD92_6801u64;
        let pte5 = 0x4018_2528_8D16_1401u64;
        let pte6 = 0x0009_DE02_DFDC_D401u64;
        let pte7 = 0xC020_CE55_6717_A401u64;

        let pte0 = PteType::from_bytes(pte0, AddressTranslationMode::Sv39);
        let pte1 = PteType::from_bytes(pte1, AddressTranslationMode::Sv39);
        let pte2 = PteType::from_bytes(pte2, AddressTranslationMode::Sv39);
        let pte3 = PteType::from_bytes(pte3, AddressTranslationMode::Sv39);
        let pte4 = PteType::from_bytes(pte4, AddressTranslationMode::Sv39);
        let pte5 = PteType::from_bytes(pte5, AddressTranslationMode::Sv39);
        let pte6 = PteType::from_bytes(pte6, AddressTranslationMode::Sv39);
        let pte7 = PteType::from_bytes(pte7, AddressTranslationMode::Sv39);

        let expected_pte0 = PteType::Branch(Pte::Sv39 {
            flags: make_bitflags!(PteFlags::{ V }),
            ppn: [122, 207, 40375447],
            pbmt: 0,
            n: false,
        });

        let expected_pte1 = PteType::Branch(Pte::Sv39 {
            flags: make_bitflags!(PteFlags::{ V }),
            ppn: [15, 414, 56696684],
            pbmt: 0,
            n: false,
        });

        let expected_pte2 = PteType::Branch(Pte::Sv39 {
            flags: make_bitflags!(PteFlags::{ V }),
            ppn: [450, 173, 17391015],
            pbmt: 0,
            n: false,
        });

        let expected_pte3 = PteType::Branch(Pte::Sv39 {
            flags: make_bitflags!(PteFlags::{ V }),
            ppn: [110, 326, 9118352],
            pbmt: 0,
            n: false,
        });

        let expected_pte4 = PteType::Branch(Pte::Sv39 {
            flags: make_bitflags!(PteFlags::{ V }),
            ppn: [154, 434, 20699708],
            pbmt: 0,
            n: false,
        });

        let expected_pte5 = PteType::Branch(Pte::Sv39 {
            flags: make_bitflags!(PteFlags::{ V }),
            ppn: [389, 418, 25318024],
            pbmt: 0,
            n: false,
        });

        let expected_pte6 = PteType::Branch(Pte::Sv39 {
            flags: make_bitflags!(PteFlags::{ V }),
            ppn: [309, 507, 10346541],
            pbmt: 0,
            n: false,
        });

        let expected_pte7 = PteType::Branch(Pte::Sv39 {
            flags: make_bitflags!(PteFlags::{ V }),
            ppn: [489, 226, 34399574],
            pbmt: 0,
            n: false,
        });

        assert_eq!(pte0, expected_pte0);
        assert_eq!(pte1, expected_pte1);
        assert_eq!(pte2, expected_pte2);
        assert_eq!(pte3, expected_pte3);
        assert_eq!(pte4, expected_pte4);
        assert_eq!(pte5, expected_pte5);
        assert_eq!(pte6, expected_pte6);
        assert_eq!(pte7, expected_pte7);
    }
}

#[test]
fn read() {
    let mem = Memory::new::<256>();
    let result = mem.read_bytes(0x8000000Fu64.into(), 4, PrivilegeMode::Machine, None);
    let expected_read = vec![0; 4];
    assert!(matches!(result, Ok(expected_read)));
}

#[test]
fn read_pmp() {
    let mut pmp = PMP::default();
    pmp.write_cfg_rv64(
        0,
        PmpCfg::new_configured(true, false, false, AddressMatch::TOR, false).to_bits() as u64,
    );
    pmp.write_addr_rv64(0, (0x90000000u64 >> 2));
    let mem = Memory::new::<256>();
    let result = mem.read_bytes(0x8000000Fu64.into(), 4, PrivilegeMode::User, Some(&pmp));
    assert!(matches!(result, Ok(expected_read)));
}

#[test]
fn read_pmp_denied() {
    let mut pmp = PMP::default();
    pmp.write_cfg_rv64(
        0,
        PmpCfg::new_configured(false, false, false, AddressMatch::TOR, false).to_bits() as u64,
    );
    pmp.write_addr_rv64(0, (0x90000000u64 >> 2));
    let mem = Memory::new::<256>();
    let result = mem.read_bytes(0x8000000Fu64.into(), 4, PrivilegeMode::User, Some(&pmp));
    assert!(matches!(result, Err(MemoryError::PmpDeniedRead)));
}

#[test]
fn read_oob() {
    let mem = Memory::new::<256>();
    let result = mem.read_bytes(0x800000FFu64.into(), 4, PrivilegeMode::Machine, None);
    assert!(matches!(result, Err(MemoryError::OutOfBoundsRead(_, _))));
}

#[test]
fn write() {
    let mut mem = Memory::new::<256>();
    let to_write = [37; 4];
    let result = mem.write_bytes(
        &to_write,
        0x8000000Fu64.into(),
        PrivilegeMode::Machine,
        None,
    );
    assert!(matches!(result, Ok(())));
    assert_eq!(
        mem.read_bytes(0x8000000Fu64.into(), 4, PrivilegeMode::Machine, None)
            .unwrap(),
        vec![37; 4]
    )
}

#[test]
fn write_pmp_denied() {
    let mut pmp = PMP::default();
    pmp.write_cfg_rv64(
        0,
        PmpCfg::new_configured(true, false, true, AddressMatch::TOR, false).to_bits() as u64,
    );
    pmp.write_addr_rv64(0, (0x90000000u64 >> 2));
    let mut mem = Memory::new::<256>();
    let to_write = [37; 4];
    let result = mem.write_bytes(
        &to_write,
        0x8000000Fu64.into(),
        PrivilegeMode::User,
        Some(&pmp),
    );
    assert!(matches!(result, Err(MemoryError::PmpDeniedWrite)));
}

#[test]
fn write_pmp() {
    let mut pmp = PMP::default();
    pmp.write_cfg_rv64(
        0,
        PmpCfg::new_configured(true, true, false, AddressMatch::TOR, false).to_bits() as u64,
    );
    pmp.write_addr_rv64(0, (0x90000000u64 >> 2));
    let mut mem = Memory::new::<256>();
    let to_write = [37; 4];
    let result = mem.write_bytes(
        &to_write,
        0x8000000Fu64.into(),
        PrivilegeMode::User,
        Some(&pmp),
    );
    assert!(matches!(result, Ok(())));
    assert_eq!(
        mem.read_bytes(0x8000000Fu64.into(), 4, PrivilegeMode::User, Some(&pmp))
            .unwrap(),
        vec![37; 4]
    )
}

#[test]
fn write_oom() {
    let mut mem = Memory::new::<256>();
    let to_write = [37; 4];
    let result = mem.write_bytes(
        &to_write,
        0x800000FFu64.into(),
        PrivilegeMode::Machine,
        None,
    );
    assert!(matches!(result, Err(MemoryError::OutOfMemory)));
}

#[test]
fn write_oob() {
    let mut mem = Memory::new::<256>();
    let to_write = [37; 4];
    let result = mem.write_bytes(
        &to_write,
        0x800001FFu64.into(),
        PrivilegeMode::Machine,
        None,
    );
    assert!(matches!(result, Err(MemoryError::OutOfBoundsWrite(_, _))));
}
