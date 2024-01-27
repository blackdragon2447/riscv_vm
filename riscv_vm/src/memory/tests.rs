use crate::{
    hart::privilege::PrivilegeMode,
    memory::{
        pmp::{AddressMatch, PmpCfg, PMP},
        MemoryError,
    },
};

use super::Memory;

mod pmp {

    use crate::memory::pmp::{AddressMatch, PmpCfg, PMP};

    #[test]
    fn rv32_pmp_read() {
        let pmp = PMP::new();
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
        let pmp = PMP::new();
        assert_eq!(pmp.read_cfg_rv32(16), 0);
    }

    #[test]
    fn rv64_pmp_read() {
        let pmp = PMP::new();
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
        let pmp = PMP::new();
        assert_eq!(pmp.read_cfg_rv64(16), 0);
    }

    #[test]
    #[should_panic(expected = "Index of 64bit pmpcfg must be even")]
    fn rv64_pmp_read_uneven() {
        let pmp = PMP::new();
        assert_eq!(pmp.read_cfg_rv64(1), 0);
    }

    #[test]
    fn rv32_write() {
        let mut pmp = PMP::new();
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
        let mut pmp = PMP::new();
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
        let mut pmp = PMP::new();
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
                (0x80000000u64.into()..0x8000001fu64.into())
            )
        );
        assert_eq!(
            ranges[1],
            (
                &PmpCfg::new_configured(true, true, false, AddressMatch::TOR, false),
                (0xB1000u64.into()..0xB1FA0u64.into())
            )
        );
        assert_eq!(
            ranges[0],
            (
                &PmpCfg::new_configured(true, false, false, AddressMatch::TOR, true),
                (0x0u64.into()..0xB1000u64.into())
            )
        );
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
    let mut pmp = PMP::new();
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
    let mut pmp = PMP::new();
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
    let mut pmp = PMP::new();
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
    let mut pmp = PMP::new();
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
