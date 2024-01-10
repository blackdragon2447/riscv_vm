use crate::{
    hart::{CsrAddress, Hart},
    memory::{address::Address, Memory},
};

pub(super) fn csrrw(hart: &mut Hart, rd: &mut i64, rs1: &i64, csr: CsrAddress) {
    *rd = hart
        .get_csr()
        .write_csr(csr.into(), *rs1 as u64, true)
        .unwrap() as i64;
}

pub(super) fn csrw(hart: &mut Hart, rs1: &i64, csr: CsrAddress) {
    hart.get_csr().write_csr(csr.into(), *rs1 as u64, false);
}

pub(super) fn csrr(hart: &mut Hart, rd: &mut i64, csr: CsrAddress) {
    *rd = hart.get_csr().set_csr(csr, 0, false) as i64;
}

pub(super) fn csrrs(hart: &mut Hart, rd: &mut i64, rs1: &i64, csr: CsrAddress) {
    *rd = hart.get_csr().set_csr(csr, *rs1 as u64, true) as i64;
}

pub(super) fn csrrc(hart: &mut Hart, rd: &mut i64, rs1: &i64, csr: CsrAddress) {
    *rd = hart.get_csr().clear_csr(csr, *rs1 as u64, true) as i64;
}

pub(super) fn csrrwi(hart: &mut Hart, rd: &mut i64, imm: u32, csr: CsrAddress) {
    *rd = hart
        .get_csr()
        .write_csr(csr.into(), imm as u64, true)
        .unwrap() as i64;
}

pub(super) fn csrwi(hart: &mut Hart, imm: u32, csr: CsrAddress) {
    hart.get_csr().write_csr(csr.into(), imm as u64, false);
}

pub(super) fn csrri(hart: &mut Hart, rd: &mut i64, csr: CsrAddress) {
    *rd = hart.get_csr().set_csr(csr, 0, false) as i64;
}

pub(super) fn csrrsi(hart: &mut Hart, rd: &mut i64, imm: u32, csr: CsrAddress) {
    *rd = hart.get_csr().set_csr(csr, imm as u64, true) as i64;
}

pub(super) fn csrrci(hart: &mut Hart, rd: &mut i64, imm: u32, csr: CsrAddress) {
    *rd = hart.get_csr().clear_csr(csr, imm as u64, true) as i64;
}
