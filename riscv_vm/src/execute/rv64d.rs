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

use softfloat_wrapper::{Float, F64};

inst!(fcvt_l_d(r) for [b64, f64]
    where [rd: int, rs1: float, rs2: float]:
{
    *rd = rs1.to_i64(rm.into(), true) as ixlen;
    Ok(ExecuteResult::Continue)
});

inst!(fcvt_lu_d(r) for [b64, f64]
    where [rd:int, rs1: float, rs2: float]:
{
    *rd = rs1.to_u64(rm.into(), true) as ixlen;
    Ok(ExecuteResult::Continue)
});

inst!(fmv_x_d(r) for [b64, f64]
    where [rd: int, rs1: float, rs2: float]:
{
    *rd = rs1.to_bits() as ixlen;
    Ok(ExecuteResult::Continue)
});

inst!(fcvt_d_l(r) for [b64, f64]
    where [rd: float, rs1: int, rs2: int]:
{
    *rd = F64::from_i64(*rs1, rm.into());
    Ok(ExecuteResult::Continue)
});

inst!(fcvt_d_lu(r) for [b64, f64]
    where [rd: float, rs1: int, rs2: int]:
{
    *rd = F64::from_u64(*rs1 as u64, rm.into());
    Ok(ExecuteResult::Continue)
});

inst!(fmv_d_x(r) for [b64, f64]
    where [rd: float, rs1: int, rs2: int]:
{
    *rd = F64::from_bits(*rs1 as u64);
    Ok(ExecuteResult::Continue)
});
