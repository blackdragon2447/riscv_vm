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

use softfloat_wrapper::{ExceptionFlags, Float, F32, F64};

inst!(fld(i_mem) for [b32, b64, f64]
    where [rd: float, rs1: int]:
{
    let bytes = mem.read_bytes(rs1.overflowing_add(imm.into()).0.into(), 8)?;
    let mut buf = [0; 8];
    buf.copy_from_slice(&bytes);
    *rd = F64::from_bits(u64::from_le_bytes(buf));
    Ok(ExecuteResult::Continue)
});

inst!(fsd(s_mem) for [b32, b64, f64]
    where [rs1: int, rs2: float]:
{
    mem.write_bytes(
        &rs2.to_bits().to_le_bytes(),
        rs1.overflowing_add(imm.into()).0.into()
    )?;
    Ok(ExecuteResult::Continue)
});

inst!(fmadd_d(r4) for [b32, b64, f64]
    where [rd: float, rs1: float, rs2: float, rs3: float]:
{
    *rd = rs1.fused_mul_add(rs2, rs3, rm.into());
    Ok(ExecuteResult::Continue)
});

inst!(fmsub_d(r4) for [b32, b64, f64]
    where [rd: float, rs1: float, rs2: float, rs3: float]:
{
    *rd = rs1.fused_mul_add(rs2, &negate(rs3), rm.into());
    Ok(ExecuteResult::Continue)
});

inst!(fnmadd_d(r4) for [b32, b64, f64]
    where [rd: float, rs1: float, rs2: float, rs3: float]:
{
    *rd = negate(rs1).fused_mul_add(rs2, &negate(rs3), rm.into());
    Ok(ExecuteResult::Continue)
});

inst!(fnmsub_d(r4) for [b32, b64, f64]
    where [rd: float, rs1: float, rs2: float, rs3: float]:
{
    *rd = negate(rs1).fused_mul_add(rs2, rs3, rm.into());
    Ok(ExecuteResult::Continue)
});

inst!(fadd_d(r) for [b32, b64, f64]
    where [rd: float, rs1: float, rs2: float]:
{
    *rd = rs1.add(rs2, rm.into());
    Ok(ExecuteResult::Continue)
});

inst!(fsub_d(r) for [b32, b64, f64]
    where [rd: float, rs1: float, rs2: float]:
{
    *rd = rs1.sub(rs2, rm.into());
    Ok(ExecuteResult::Continue)
});

inst!(fmul_d(r) for [b32, b64, f64]
    where [rd: float, rs1: float, rs2: float]:
{
    *rd = rs1.mul(rs2, rm.into());
    Ok(ExecuteResult::Continue)
});

inst!(fdiv_d(r) for [b32, b64, f64]
    where [rd: float, rs1: float, rs2: float]:
{
    *rd = rs1.div(rs2, rm.into());
    Ok(ExecuteResult::Continue)
});

inst!(fsqrt_d(r) for [b32, b64, f64]
    where [rd: float, rs1: float, rs2: float]:
{
    *rd = rs1.sqrt(rm.into());
    Ok(ExecuteResult::Continue)
});

inst!(fsgnj_d(r) for [b32, b64, f64]
    where [rd: float, rs1: float, rs2: float]:
{
    *rd = rs1.clone();
    rd.set_sign(rs2.sign());
    Ok(ExecuteResult::Continue)
});

inst!(fsgnjn_d(r) for [b32, b64, f64]
    where [rd: float, rs1: float, rs2: float]:
{
    *rd = rs1.clone();
    rd.set_sign(!rs2.sign());
    Ok(ExecuteResult::Continue)
});

inst!(fsgnjx_d(r) for [b32, b64, f64]
    where [rd: float, rs1: float, rs2: float]:
{
    *rd = rs1.clone();
    rd.set_sign(rs1.sign() ^ rs2.sign());
    Ok(ExecuteResult::Continue)
});

inst!(fmin_d(r) for [b32, b64, f64]
    where [rd: float, rs1: float, rs2: float]:
{
    *rd = if (rs1.is_nan() && rs2.is_nan()) {
        F64::quiet_nan()
    } else if (rs1.is_negative_zero() && rs2.is_positive_zero())
        || (rs1.is_positive_zero() && rs2.is_negative_zero())
    {
        F64::negative_zero()
    } else {
        if rs1.lt_quiet(rs2) {
            if !rs1.is_nan() {
                *rs1
            } else {
                *rs2
            }
        } else {
            if !rs2.is_nan() {
                *rs2
            } else {
                *rs1
            }
        }
    };
    Ok(ExecuteResult::Continue)
});

inst!(fmax_d(r) for [b32, b64, f64]
    where [rd: float, rs1: float, rs2: float]:
{
    *rd = if (rs1.is_nan() && rs2.is_nan()) {
        F64::quiet_nan()
    } else if (rs1.is_negative_zero() && rs2.is_positive_zero())
        || (rs1.is_positive_zero() && rs2.is_negative_zero())
    {
        F64::positive_zero()
    } else {
        if rs1.lt_quiet(rs2) {
            if !rs2.is_nan() {
                *rs2
            } else {
                *rs1
            }
        } else {
            if !rs1.is_nan() {
                *rs1
            } else {
                *rs2
            }
        }
    };
    Ok(ExecuteResult::Continue)
});

pub(super) fn fcvt_s_d_32(
    pd: Address,
    rd: &mut F32,
    rs1: &F64,
    rs2: &F64,
    rm: RoundingMode,
) -> Result<ExecuteResult, ExecuteError> {
    *rd = rs1.to_f32(rm.into());
    Ok(ExecuteResult::Continue)
}

pub(super) fn fcvt_s_d_64(
    pd: Address,
    rd: &mut F32,
    rs1: &F64,
    rs2: &F64,
    rm: RoundingMode,
) -> Result<ExecuteResult, ExecuteError> {
    *rd = rs1.to_f32(rm.into());
    Ok(ExecuteResult::Continue)
}

pub(super) fn fcvt_d_s_32(
    pd: Address,
    rd: &mut F64,
    rs1: &F32,
    rs2: &F32,
    rm: RoundingMode,
) -> Result<ExecuteResult, ExecuteError> {
    *rd = rs1.to_f64(rm.into());
    Ok(ExecuteResult::Continue)
}

pub(super) fn fcvt_d_s_64(
    pd: Address,
    rd: &mut F64,
    rs1: &F32,
    rs2: &F32,
    rm: RoundingMode,
) -> Result<ExecuteResult, ExecuteError> {
    *rd = rs1.to_f64(rm.into());
    Ok(ExecuteResult::Continue)
}

inst!(feq_d(r) for [b32, b64, f64]
    where [rd: int, rs1: float, rs2: float]:
{
    *rd = rs1.eq(rs2) as ixlen;
    Ok(ExecuteResult::Continue)
});

inst!(flt_d(r) for [b32, b64, f64]
    where [rd: int, rs1: float, rs2:float]:
{
    *rd = rs1.lt(rs2) as ixlen;
    Ok(ExecuteResult::Continue)
});

inst!(fle_d(r) for [b32, b64, f64]
    where [rd: int, rs1: float, rs2:float]:
{
    *rd = rs1.le(rs2) as ixlen;
    Ok(ExecuteResult::Continue)
});

inst!(fclass_d(r) for [b32, b64, f64]
    where [rd: int, rs1: float, rs2: float]:
{
    *rd = if rs1.is_negative_infinity() {
        0x1 << 0
    } else if rs1.is_negative_normal() {
        0x1 << 1
    } else if rs1.is_negative_subnormal() {
        0x1 << 2
    } else if rs1.is_negative_zero() {
        0x1 << 3
    } else if rs1.is_positive_zero() {
        0x1 << 4
    } else if rs1.is_positive_subnormal() {
        0x1 << 5
    } else if rs1.is_positive_normal() {
        0x1 << 6
    } else if rs1.is_positive_infinity() {
        0x1 << 7
    } else if rs1.is_signaling_nan() {
        0x1 << 8
    } else {
        0x1 << 9
    };

    Ok(ExecuteResult::Continue)
});

inst!(fcvt_w_d(r) for [b32, b64, f64]
    where [rd: int, rs1: float, rs2: float]:
{
    *rd = rs1.to_i32(rm.into(), true) as ixlen;
    Ok(ExecuteResult::Continue)
});

inst!(fcvt_wu_d(r) for [b32, b64, f64]
    where [rd: int, rs1: float, rs2: float]:
{
    *rd = rs1.to_u32(rm.into(), true) as i32 as ixlen;
    Ok(ExecuteResult::Continue)
});

inst!(fcvt_d_w(r) for [b32, b64, f64]
    where [rd: float, rs1: int, rs2: int]:
{
    *rd = F64::from_i32(*rs1 as i32, rm.into());
    Ok(ExecuteResult::Continue)
});

inst!(fcvt_d_wu(r) for [b32, b64, f64]
    where [rd: float, rs1: int, rs2: int]:
{
    *rd = F64::from_u32(*rs1 as u32, rm.into());
    Ok(ExecuteResult::Continue)
});

fn negate<F: Float + Clone>(num: &F) -> F {
    let mut num = num.clone();
    num.set_sign(!num.sign());
    num
}

#[test]
fn negations() {
    assert!(F64::eq(
        &F64::from_f64(-3.14),
        &negate(&F64::from_f64(3.14))
    ));
    assert!(F64::eq(
        &F64::from_f64(-0.0000000000000001),
        &negate(&F64::from_f64(0.0000000000000001))
    ));
    assert!(F64::eq(
        &F64::from_f64(-37e9),
        &negate(&F64::from_f64(37e9))
    ));
    assert!(F64::eq(&F64::from_f64(-1.0), &negate(&F64::from_f64(1.0))));
    assert!(F64::eq(
        &F64::from_f64(-1e38),
        &negate(&F64::from_f64(1e38))
    ));
    assert!(F64::eq(
        &F64::negative_zero(),
        &negate(&F64::positive_zero())
    ));
    assert!(F64::eq(
        &F64::negative_infinity(),
        &negate(&F64::positive_infinity())
    ));
}
