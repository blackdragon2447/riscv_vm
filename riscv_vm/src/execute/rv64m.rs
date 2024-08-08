#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

use riscv_vm_macros::inst;

use crate::memory::{address::Address, Memory};

use super::{ExecuteError, ExecuteResult};

inst!(mulw(r) for [b64]
    where [rd: int, rs1: int, rs2: int]:
{
    *rd = (*rs1 as i32).overflowing_mul(*rs2 as i32).0 as ixlen;
    Ok(ExecuteResult::Continue)
});

inst!(divw(r) for [b64]
    where [rd: int, rs1: int, rs2: int]:
{
    if (*rs2 == 0) {
        *rd = -1;
    } else {
        *rd = (*rs1 as i32).overflowing_div(*rs2 as i32).0 as ixlen;
    }
    Ok(ExecuteResult::Continue)
});

inst!(divuw(r) for [b64]
    where [rd: int, rs1: int, rs2: int]:
{
    if (*rs2 == 0) {
        *rd = -1;
    } else {
        *rd = ((*rs1 as u32) / (*rs2 as u32)) as i32 as ixlen;
    }
    Ok(ExecuteResult::Continue)
});

inst!(remw(r) for [b64]
    where [rd: int, rs1: int, rs2: int]:
{
    if (*rs2 == 0) {
        *rd = *rs1;
    } else {
        *rd = (*rs1 as i32).overflowing_rem(*rs2 as i32).0 as ixlen;
    }
    Ok(ExecuteResult::Continue)
});

inst!(remuw(r) for [b64]
    where [rd: int, rs1: int, rs2: int]:
{
    if (*rs2 == 0) {
        *rd = *rs1;
    } else {
        *rd = ((*rs1 as u32) % (*rs2 as u32)) as i32 as ixlen;
    }
    Ok(ExecuteResult::Continue)
});
