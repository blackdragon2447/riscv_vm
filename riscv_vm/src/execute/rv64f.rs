#![allow(clippy::useless_conversion)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

use riscv_vm_macros::inst;

use std::cmp::Ordering;

use crate::{
    decode::instruction::RoundingMode,
    hart::trap::Exception,
    memory::{address::Address, Memory, MemoryWindow},
};

use super::{ExecuteError, ExecuteResult};

use softfloat_wrapper::{Float, F32};

inst!(fcvt_l_s(r) for [b64, f32]
    where [rd: int, rs1: float, rs2: float]:
{
    *rd = rs1.to_i64(rm.into(), true) as ixlen;
    Ok(ExecuteResult::Continue)
});

inst!(fcvt_lu_s(r) for [b64, f32]
    where [rd:int, rs1: float, rs2: float]:
{
    *rd = rs1.to_u64(rm.into(), true) as ixlen;
    Ok(ExecuteResult::Continue)
});

inst!(fcvt_s_l(r) for [b64, f32]
    where [rd: float, rs1: int, rs2: int]:
{
    *rd = F32::from_i64(*rs1, rm.into());
    Ok(ExecuteResult::Continue)
});

inst!(fcvt_s_lu(r) for [b64, f32]
    where [rd: float, rs1: int, rs2: int]:
{
    *rd = F32::from_u64(*rs1 as u64, rm.into());
    Ok(ExecuteResult::Continue)
});
