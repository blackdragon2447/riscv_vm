use riscv_vm_macros::inst;

use crate::memory::{address::Address, Memory};

use super::{ExecuteError, ExecuteResult};

inst!(mulw(r) for [64]: {
    *rd = (*rs1 as i32).overflowing_mul(*rs2 as i32).0 as ixlen;
    Ok(ExecuteResult::Continue)
});

inst!(divw(r) for [64]: {
    if (*rs2 == 0) {
        *rd = -1;
    } else {
        *rd = (*rs1 as i32).overflowing_div(*rs2 as i32).0 as ixlen;
    }
    Ok(ExecuteResult::Continue)
});

inst!(divuw(r) for [64]: {
    if (*rs2 == 0) {
        *rd = -1;
    } else {
        *rd = ((*rs1 as u32) / (*rs2 as u32)) as i32 as ixlen;
    }
    Ok(ExecuteResult::Continue)
});

inst!(remw(r) for [64]: {
    if (*rs2 == 0) {
        *rd = *rs1;
    } else {
        *rd = (*rs1 as i32).overflowing_rem(*rs2 as i32).0 as ixlen;
    }
    Ok(ExecuteResult::Continue)
});

inst!(remuw(r) for [64]: {
    if (*rs2 == 0) {
        *rd = *rs1;
    } else {
        *rd = ((*rs1 as u32) % (*rs2 as u32)) as i32 as ixlen;
    }
    Ok(ExecuteResult::Continue)
});
