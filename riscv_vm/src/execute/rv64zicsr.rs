use crate::{
    hart::{CsrAddress, Hart},
    memory::{address::Address, Memory},
};

use super::{ExecuteError, ExecuteResult};

pub(super) fn csrrw(
    hart: &mut Hart,
    rd: &mut i64,
    rs1: &i64,
    csr: CsrAddress,
) -> Result<ExecuteResult, ExecuteError> {
    *rd = hart.get_csr().write_csr(csr, *rs1 as u64, true).unwrap() as i64;
    Ok(ExecuteResult::Continue)
}

pub(super) fn csrw(
    hart: &mut Hart,
    rs1: &i64,
    csr: CsrAddress,
) -> Result<ExecuteResult, ExecuteError> {
    hart.get_csr().write_csr(csr, *rs1 as u64, false);
    Ok(ExecuteResult::Continue)
}

pub(super) fn csrr(
    hart: &mut Hart,
    rd: &mut i64,
    csr: CsrAddress,
) -> Result<ExecuteResult, ExecuteError> {
    *rd = hart.get_csr().set_csr(csr, 0, false) as i64;
    Ok(ExecuteResult::Continue)
}

pub(super) fn csrrs(
    hart: &mut Hart,
    rd: &mut i64,
    rs1: &i64,
    csr: CsrAddress,
) -> Result<ExecuteResult, ExecuteError> {
    *rd = hart.get_csr().set_csr(csr, *rs1 as u64, true) as i64;
    Ok(ExecuteResult::Continue)
}

pub(super) fn csrrc(
    hart: &mut Hart,
    rd: &mut i64,
    rs1: &i64,
    csr: CsrAddress,
) -> Result<ExecuteResult, ExecuteError> {
    *rd = hart.get_csr().clear_csr(csr, *rs1 as u64, true) as i64;
    Ok(ExecuteResult::Continue)
}

pub(super) fn csrrwi(
    hart: &mut Hart,
    rd: &mut i64,
    imm: u32,
    csr: CsrAddress,
) -> Result<ExecuteResult, ExecuteError> {
    *rd = hart.get_csr().write_csr(csr, imm as u64, true).unwrap() as i64;
    Ok(ExecuteResult::Continue)
}

pub(super) fn csrwi(
    hart: &mut Hart,
    imm: u32,
    csr: CsrAddress,
) -> Result<ExecuteResult, ExecuteError> {
    hart.get_csr().write_csr(csr, imm as u64, false);
    Ok(ExecuteResult::Continue)
}

pub(super) fn csrri(
    hart: &mut Hart,
    rd: &mut i64,
    csr: CsrAddress,
) -> Result<ExecuteResult, ExecuteError> {
    *rd = hart.get_csr().set_csr(csr, 0, false) as i64;
    Ok(ExecuteResult::Continue)
}

pub(super) fn csrrsi(
    hart: &mut Hart,
    rd: &mut i64,
    imm: u32,
    csr: CsrAddress,
) -> Result<ExecuteResult, ExecuteError> {
    *rd = hart.get_csr().set_csr(csr, imm as u64, true) as i64;
    Ok(ExecuteResult::Continue)
}

pub(super) fn csrrci(
    hart: &mut Hart,
    rd: &mut i64,
    imm: u32,
    csr: CsrAddress,
) -> Result<ExecuteResult, ExecuteError> {
    *rd = hart.get_csr().clear_csr(csr, imm as u64, true) as i64;
    Ok(ExecuteResult::Continue)
}
