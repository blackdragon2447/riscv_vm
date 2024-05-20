#![allow(non_camel_case_types)]
use riscv_vm_macros::inst;

use crate::{
    hart::Hart,
    memory::{address::Address, Memory},
};

use super::{ExecuteError, ExecuteResult};

inst!(mul(r) for [32, 64]: {
    *rd = rs1.overflowing_mul(*rs2).0;
    Ok(ExecuteResult::Continue)
});

inst!(mulh(r) for [32, 64]: {
    *rd = ((*rs1 as iexlen).overflowing_mul(*rs2 as iexlen).0 >> xlen) as ixlen;
    Ok(ExecuteResult::Continue)
});

inst!(mulhsu(r) for [32, 64]: {
    *rd = ((*rs1 as iexlen)
        .overflowing_mul(*rs2 as uxlen as uexlen as iexlen)
        .0
        >> xlen) as ixlen;
    Ok(ExecuteResult::Continue)
});

inst!(mulhu(r) for [32, 64]: {

    *rd = ((*rs1 as uxlen as uexlen).overflowing_mul(*rs2 as uxlen as uexlen).0 >> xlen) as ixlen;
    Ok(ExecuteResult::Continue)
});

inst!(div(r) for [32, 64]: {
    if (*rs2 == 0) {
        *rd = -1;
    } else {
        *rd = (*rs1).overflowing_div(*rs2).0;
    }
    Ok(ExecuteResult::Continue)
});

inst!(divu(r) for [32, 64]: {
    if (*rs2 == 0) {
        *rd = uxlen::MAX as ixlen;
    } else {
        *rd = ((*rs1 as uxlen) / (*rs2 as uxlen)) as ixlen;
    }
    Ok(ExecuteResult::Continue)
});

inst!(rem(r) for [32, 64]: {
    if (*rs2 == 0) {
        *rd = *rs1;
    } else {
        *rd = (*rs1).overflowing_rem(*rs2).0;
    }
    Ok(ExecuteResult::Continue)
});

inst!(remu(r) for [32, 64]: {
    if (*rs2 == 0) {
        *rd = *rs1;
    } else {
        *rd = ((*rs1 as uxlen) % (*rs2 as uxlen)) as ixlen;
    }
    Ok(ExecuteResult::Continue)
});
