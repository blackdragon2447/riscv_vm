use crate::{
    hart::Hart,
    memory::{address::Address, Memory},
};

use super::{ExecuteError, ExecuteResult};

pub(super) fn mul(
    _: Address,
    rd: &mut i64,
    rs1: &i64,
    rs2: &i64,
) -> Result<ExecuteResult, ExecuteError> {
    *rd = rs1.overflowing_mul(*rs2).0;
    Ok(ExecuteResult::Continue)
}

pub(super) fn mulh(
    _: Address,
    rd: &mut i64,
    rs1: &i64,
    rs2: &i64,
) -> Result<ExecuteResult, ExecuteError> {
    *rd = ((*rs1 as i128).overflowing_mul(*rs2 as i128).0 >> 64) as i64;
    Ok(ExecuteResult::Continue)
}

pub(super) fn mulhsu(
    _: Address,
    rd: &mut i64,
    rs1: &i64,
    rs2: &i64,
) -> Result<ExecuteResult, ExecuteError> {
    *rd = ((*rs1 as i128)
        .overflowing_mul(*rs2 as u64 as u128 as i128)
        .0
        >> 64) as i64;
    Ok(ExecuteResult::Continue)
}

pub(super) fn mulhu(
    _: Address,
    rd: &mut i64,
    rs1: &i64,
    rs2: &i64,
) -> Result<ExecuteResult, ExecuteError> {
    *rd = ((*rs1 as u64 as u128).overflowing_mul(*rs2 as u64 as u128).0 >> 64) as i64;
    Ok(ExecuteResult::Continue)
}

pub(super) fn div(
    _: Address,
    rd: &mut i64,
    rs1: &i64,
    rs2: &i64,
) -> Result<ExecuteResult, ExecuteError> {
    if (*rs2 == 0) {
        *rd = -1;
    } else {
        *rd = (*rs1).overflowing_div(*rs2).0;
    }
    Ok(ExecuteResult::Continue)
}

pub(super) fn divu(
    _: Address,
    rd: &mut i64,
    rs1: &i64,
    rs2: &i64,
) -> Result<ExecuteResult, ExecuteError> {
    if (*rs2 == 0) {
        *rd = u64::MAX as i64;
    } else {
        *rd = ((*rs1 as u64) / (*rs2 as u64)) as i64;
    }
    Ok(ExecuteResult::Continue)
}

pub(super) fn rem(
    _: Address,
    rd: &mut i64,
    rs1: &i64,
    rs2: &i64,
) -> Result<ExecuteResult, ExecuteError> {
    if (*rs2 == 0) {
        *rd = *rs1;
    } else {
        *rd = (*rs1).overflowing_rem(*rs2).0;
    }
    Ok(ExecuteResult::Continue)
}

pub(super) fn remu(
    _: Address,
    rd: &mut i64,
    rs1: &i64,
    rs2: &i64,
) -> Result<ExecuteResult, ExecuteError> {
    if (*rs2 == 0) {
        *rd = *rs1;
    } else {
        *rd = ((*rs1 as u64) % (*rs2 as u64)) as i64;
    }
    Ok(ExecuteResult::Continue)
}
