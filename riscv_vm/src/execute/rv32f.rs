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

use softfloat_wrapper::{ExceptionFlags, Float, F32};

inst!(flw(i_mem) for [b32, b64, f32]
    where [rd: float, rs1: int]:
{
    let bytes = mem.read_bytes(rs1.overflowing_add(imm.into()).0.into(), 4)?;
    let mut buf = [0; 4];
    buf.copy_from_slice(&bytes);
    *rd = F32::from_bits(u32::from_le_bytes(buf));
    Ok(ExecuteResult::Continue)
});

inst!(fsw(s_mem) for [b32, b64, f32]
    where [rs1: int, rs2: float]:
{
    mem.write_bytes(
        &rs2.to_bits().to_le_bytes(),
        rs1.overflowing_add(imm.into()).0.into()
    )?;
    Ok(ExecuteResult::Continue)
});

inst!(fmadd_s(r4) for [b32, b64, f32]
    where [rd: float, rs1: float, rs2: float, rs3: float]:
{
    *rd = rs1.fused_mul_add(rs2, rs3, rm.into());
    Ok(ExecuteResult::Continue)
});

inst!(fmsub_s(r4) for [b32, b64, f32]
    where [rd: float, rs1: float, rs2: float, rs3: float]:
{
    *rd = rs1.fused_mul_add(rs2, &negate(rs3), rm.into());
    Ok(ExecuteResult::Continue)
});

inst!(fnmadd_s(r4) for [b32, b64, f32]
    where [rd: float, rs1: float, rs2: float, rs3: float]:
{
    *rd = negate(rs1).fused_mul_add(rs2, &negate(rs3), rm.into());
    Ok(ExecuteResult::Continue)
});

inst!(fnmsub_s(r4) for [b32, b64, f32]
    where [rd: float, rs1: float, rs2: float, rs3: float]:
{
    *rd = negate(rs1).fused_mul_add(rs2, rs3, rm.into());
    Ok(ExecuteResult::Continue)
});

inst!(fadd_s(r) for [b32, b64, f32]
    where [rd: float, rs1: float, rs2: float]:
{
    *rd = rs1.add(rs2, rm.into());
    Ok(ExecuteResult::Continue)
});

inst!(fsub_s(r) for [b32, b64, f32]
    where [rd: float, rs1: float, rs2: float]:
{
    *rd = rs1.sub(rs2, rm.into());
    Ok(ExecuteResult::Continue)
});

inst!(fmul_s(r) for [b32, b64, f32]
    where [rd: float, rs1: float, rs2: float]:
{
    *rd = rs1.mul(rs2, rm.into());
    Ok(ExecuteResult::Continue)
});

inst!(fdiv_s(r) for [b32, b64, f32]
    where [rd: float, rs1: float, rs2: float]:
{
    *rd = rs1.div(rs2, rm.into());
    Ok(ExecuteResult::Continue)
});

inst!(fsqrt_s(r) for [b32, b64, f32]
    where [rd: float, rs1: float, rs2: float]:
{
    *rd = rs1.sqrt(rm.into());
    Ok(ExecuteResult::Continue)
});

inst!(fsgnj_s(r) for [b32, b64, f32]
    where [rd: float, rs1: float, rs2: float]:
{
    *rd = rs1.clone();
    rd.set_sign(rs2.sign());
    Ok(ExecuteResult::Continue)
});

inst!(fsgnjn_s(r) for [b32, b64, f32]
    where [rd: float, rs1: float, rs2: float]:
{
    *rd = rs1.clone();
    rd.set_sign(!rs2.sign());
    Ok(ExecuteResult::Continue)
});

inst!(fsgnjx_s(r) for [b32, b64, f32]
    where [rd: float, rs1: float, rs2: float]:
{
    *rd = rs1.clone();
    rd.set_sign(rs1.sign() ^ rs2.sign());
    Ok(ExecuteResult::Continue)
});

inst!(fmin_s(r) for [b32, b64, f32]
    where [rd: float, rs1: float, rs2: float]:
{
    // let mut flags = ExceptionFlags::default();
    // flags.get();
    // let invalid = flags.is_invalid();
    // *rd = match rs1.compare(rs2) {
    //     Some(Ordering::Less) => *rs1,
    //     Some(Ordering::Equal) => *rs1,
    //     Some(Ordering::Greater) => *rs2,
    //     None if rs1.is_nan() && rs2.is_nan() => F32::quiet_nan(),
    //     None if rs1.is_nan() && !rs2.is_nan() => *rs2,
    //     _ => *rs1,
    // };
    // if(!rs1.is_signaling_nan() && !rs2.is_signaling_nan() && !invalid) {
    //     let flags = ExceptionFlags::from_bits(flags.to_bits() & !(1 << 4));
    //     flags.set();
    // }
    *rd = if (rs1.is_nan() && rs2.is_nan()) {
        F32::quiet_nan()
    } else if (rs1.is_negative_zero() && rs2.is_positive_zero())
        || (rs1.is_positive_zero() && rs2.is_negative_zero())
    {
        F32::negative_zero()
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

inst!(fmax_s(r) for [b32, b64, f32]
    where [rd: float, rs1: float, rs2: float]:
{
    // let mut flags = ExceptionFlags::default();
    // flags.get();
    // let invalid = flags.is_invalid();
    // *rd = match rs1.compare(rs2) {
    //     Some(Ordering::Less) => *rs2,
    //     Some(Ordering::Equal) => *rs2,
    //     Some(Ordering::Greater) => *rs1,
    //     None if rs1.is_nan() && rs2.is_nan() => F32::quiet_nan(),
    //     None if rs1.is_nan() && !rs2.is_nan() => *rs2,
    //     _ => *rs1,
    // };
    // if(!rs1.is_signaling_nan() && !rs2.is_signaling_nan() && !invalid) {
    //     let flags = ExceptionFlags::from_bits(flags.to_bits() & !(1 << 4));
    //     flags.set();
    // }
    *rd = if (rs1.is_nan() && rs2.is_nan()) {
        F32::quiet_nan()
    } else if (rs1.is_negative_zero() && rs2.is_positive_zero())
        || (rs1.is_positive_zero() && rs2.is_negative_zero())
    {
        F32::positive_zero()
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

inst!(fcvt_w_s(r) for [b32, b64, f32]
    where [rd: int, rs1: float, rs2: float]:
{
    *rd = rs1.to_i32(rm.into(), true) as ixlen;
    Ok(ExecuteResult::Continue)
});

inst!(fcvt_wu_s(r) for [b32, b64, f32]
    where [rd:int, rs1: float, rs2: float]:
{
    *rd = rs1.to_u32(rm.into(), true) as i32 as ixlen;
    Ok(ExecuteResult::Continue)
});

inst!(fmv_x_w(r) for [b32, f32]
    where [rd: int, rs1: float, rs2: float]:
{
    *rd = rs1.to_bits() as ixlen;
    Ok(ExecuteResult::Continue)
});

inst!(fmv_x_w(r) for [b64, f32]
    where [rd: int, rs1: float, rs2: float]:
{
    *rd = rs1.to_bits() as ixlen;
    let top = ((rs1.sign() as ixlen) << 63) >> 31;
    *rd |= top;
    Ok(ExecuteResult::Continue)
});

inst!(feq_s(r) for [b32, b64, f32]
    where [rd: int, rs1: float, rs2: float]:
{
    *rd = rs1.eq(rs2) as ixlen;
    Ok(ExecuteResult::Continue)
});

inst!(flt_s(r) for [b32, b64, f32]
    where [rd: int, rs1: float, rs2:float]:
{
    *rd = rs1.lt(rs2) as ixlen;
    Ok(ExecuteResult::Continue)
});

inst!(fle_s(r) for [b32, b64, f32]
    where [rd: int, rs1: float, rs2:float]:
{
    *rd = rs1.le(rs2) as ixlen;
    Ok(ExecuteResult::Continue)
});

inst!(fclass_s(r) for [b32, b64, f32]
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

inst!(fcvt_s_w(r) for [b32, b64, f32]
    where [rd: float, rs1: int, rs2: int]:
{
    *rd = F32::from_i32(*rs1 as i32, rm.into());
    Ok(ExecuteResult::Continue)
});

inst!(fcvt_s_wu(r) for [b32, b64, f32]
    where [rd: float, rs1: int, rs2: int]:
{
    *rd = F32::from_u32(*rs1 as u32, rm.into());
    Ok(ExecuteResult::Continue)
});

inst!(fmv_w_x(r) for [b32, b64, f32]
    where [rd: float, rs1: int, rs2: int]:
{
    *rd = F32::from_bits(*rs1 as u32);
    Ok(ExecuteResult::Continue)
});

fn negate<F: Float + Clone>(num: &F) -> F {
    let mut num = num.clone();
    num.set_sign(!num.sign());
    num
}

#[test]
fn negations() {
    assert!(F32::eq(
        &F32::from_f32(-3.14),
        &negate(&F32::from_f32(3.14))
    ));
    assert!(F32::eq(
        &F32::from_f32(-0.0000000000000001),
        &negate(&F32::from_f32(0.0000000000000001))
    ));
    assert!(F32::eq(
        &F32::from_f32(-37e9),
        &negate(&F32::from_f32(37e9))
    ));
    assert!(F32::eq(&F32::from_f32(-1.0), &negate(&F32::from_f32(1.0))));
    assert!(F32::eq(
        &F32::from_f32(-1e38),
        &negate(&F32::from_f32(1e38))
    ));
    assert!(F32::eq(
        &F32::negative_zero(),
        &negate(&F32::positive_zero())
    ));
    assert!(F32::eq(
        &F32::negative_infinity(),
        &negate(&F32::positive_infinity())
    ));
}
