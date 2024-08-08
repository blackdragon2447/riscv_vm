#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

use riscv_vm_macros::inst;

use crate::{
    hart::Hart,
    memory::{address::Address, Memory},
};

use super::{ExecuteError, ExecuteResult};

inst!(mul(r) for [b32, b64]
    where [rd: int, rs1: int, rs2: int]:
{
    *rd = rs1.overflowing_mul(*rs2).0;
    Ok(ExecuteResult::Continue)
});

inst!(mulh(r) for [b32, b64]
    where [rd: int, rs1: int, rs2: int]:
{
    *rd = ((*rs1 as iexlen).overflowing_mul(*rs2 as iexlen).0 >> xlen) as ixlen;
    Ok(ExecuteResult::Continue)
});

inst!(mulhsu(r) for [b32, b64]
    where [rd: int, rs1: int, rs2: int]:
{
    *rd = ((*rs1 as iexlen)
        .overflowing_mul(*rs2 as uxlen as uexlen as iexlen)
        .0
        >> xlen) as ixlen;
    Ok(ExecuteResult::Continue)
});

inst!(mulhu(r) for [b32, b64]
    where [rd: int, rs1: int, rs2: int]:
{

    *rd = ((*rs1 as uxlen as uexlen).overflowing_mul(*rs2 as uxlen as uexlen).0 >> xlen) as ixlen;
    Ok(ExecuteResult::Continue)
});

inst!(div(r) for [b32, b64]
    where [rd: int, rs1: int, rs2: int]:
{
    if (*rs2 == 0) {
        *rd = -1;
    } else {
        *rd = (*rs1).overflowing_div(*rs2).0;
    }
    Ok(ExecuteResult::Continue)
});

inst!(divu(r) for [b32, b64]
    where [rd: int, rs1: int, rs2: int]:
{
    if (*rs2 == 0) {
        *rd = uxlen::MAX as ixlen;
    } else {
        *rd = ((*rs1 as uxlen) / (*rs2 as uxlen)) as ixlen;
    }
    Ok(ExecuteResult::Continue)
});

inst!(rem(r) for [b32, b64]
    where [rd: int, rs1: int, rs2: int]:
{
    if (*rs2 == 0) {
        *rd = *rs1;
    } else {
        *rd = (*rs1).overflowing_rem(*rs2).0;
    }
    Ok(ExecuteResult::Continue)
});

inst!(remu(r) for [b32, b64]
    where [rd: int, rs1: int, rs2: int]:
{
    if (*rs2 == 0) {
        *rd = *rs1;
    } else {
        *rd = ((*rs1 as uxlen) % (*rs2 as uxlen)) as ixlen;
    }
    Ok(ExecuteResult::Continue)
});
