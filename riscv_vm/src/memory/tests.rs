use crate::memory::pmp::{AddressMatch, PmpCfg};

use super::pmp::PMP;

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
