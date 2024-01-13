use crate::{
    hart::Hart,
    memory::{address::Address, Memory},
};

use super::{ExecuteError, ExecuteResult};

pub(super) fn mulw(
    _: &Hart,
    rd: &mut i64,
    rs1: &i64,
    rs2: &i64,
) -> Result<ExecuteResult, ExecuteError> {
    *rd = (*rs1 as i32).overflowing_mul(*rs2 as i32).0 as i64;
    Ok(ExecuteResult::Continue)
}

pub(super) fn divw(
    _: &Hart,
    rd: &mut i64,
    rs1: &i64,
    rs2: &i64,
) -> Result<ExecuteResult, ExecuteError> {
    *rd = ((*rs1 as i32) / (*rs2 as i32)) as i64;
    Ok(ExecuteResult::Continue)
}

pub(super) fn divuw(
    _: &Hart,
    rd: &mut i64,
    rs1: &i64,
    rs2: &i64,
) -> Result<ExecuteResult, ExecuteError> {
    *rd = ((*rs1 as u32) / (*rs2 as u32)) as i32 as i64;
    Ok(ExecuteResult::Continue)
}

pub(super) fn remw(
    _: &Hart,
    rd: &mut i64,
    rs1: &i64,
    rs2: &i64,
) -> Result<ExecuteResult, ExecuteError> {
    *rd = ((*rs1 as i32) % (*rs2 as i32)) as i64;
    Ok(ExecuteResult::Continue)
}

pub(super) fn remuw(
    _: &Hart,
    rd: &mut i64,
    rs1: &i64,
    rs2: &i64,
) -> Result<ExecuteResult, ExecuteError> {
    *rd = ((*rs1 as u32) % (*rs2 as u32)) as i32 as i64;
    Ok(ExecuteResult::Continue)
}
