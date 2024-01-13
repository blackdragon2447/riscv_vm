use crate::{
    hart::Hart,
    memory::{address::Address, Memory},
};

use super::{ExecuteError, ExecuteResult};

pub(super) fn mul(
    _: &Hart,
    rd: &mut i64,
    rs1: &i64,
    rs2: &i64,
) -> Result<ExecuteResult, ExecuteError> {
    *rd = rs1.overflowing_mul(*rs2).0;
    Ok(ExecuteResult::Continue)
}

pub(super) fn mulh(
    _: &Hart,
    rd: &mut i64,
    rs1: &i64,
    rs2: &i64,
) -> Result<ExecuteResult, ExecuteError> {
    *rd = ((*rs1 as i128).overflowing_mul((*rs2 as i128)).0 >> 64) as i64;
    Ok(ExecuteResult::Continue)
}

pub(super) fn mulhsu(
    _: &Hart,
    rd: &mut i64,
    rs1: &i64,
    rs2: &i64,
) -> Result<ExecuteResult, ExecuteError> {
    *rd = ((*rs1 as i128).overflowing_mul((*rs2 as u128 as i128)).0 >> 64) as i64;
    Ok(ExecuteResult::Continue)
}

pub(super) fn mulhu(
    _: &Hart,
    rd: &mut i64,
    rs1: &i64,
    rs2: &i64,
) -> Result<ExecuteResult, ExecuteError> {
    *rd = ((*rs1 as u128).overflowing_mul((*rs2 as u128)).0 >> 64) as i64;
    Ok(ExecuteResult::Continue)
}

pub(super) fn div(
    _: &Hart,
    rd: &mut i64,
    rs1: &i64,
    rs2: &i64,
) -> Result<ExecuteResult, ExecuteError> {
    *rd = rs1 / rs2;
    Ok(ExecuteResult::Continue)
}

pub(super) fn divu(
    _: &Hart,
    rd: &mut i64,
    rs1: &i64,
    rs2: &i64,
) -> Result<ExecuteResult, ExecuteError> {
    *rd = ((*rs1 as u64) / (*rs2 as u64)) as i64;
    Ok(ExecuteResult::Continue)
}

pub(super) fn rem(
    _: &Hart,
    rd: &mut i64,
    rs1: &i64,
    rs2: &i64,
) -> Result<ExecuteResult, ExecuteError> {
    *rd = rs1 % rs2;
    Ok(ExecuteResult::Continue)
}

pub(super) fn remu(
    _: &Hart,
    rd: &mut i64,
    rs1: &i64,
    rs2: &i64,
) -> Result<ExecuteResult, ExecuteError> {
    *rd = ((*rs1 as u64) % (*rs2 as u64)) as i64;
    Ok(ExecuteResult::Continue)
}
