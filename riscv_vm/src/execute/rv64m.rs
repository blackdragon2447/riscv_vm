use crate::{
    hart::Hart,
    memory::{address::Address, Memory},
};

pub(super) fn mulw(_: &Hart, rd: &mut i64, rs1: &i64, rs2: &i64) {
    *rd = (*rs1 as i32).overflowing_mul(*rs2 as i32).0 as i64;
}

pub(super) fn divw(_: &Hart, rd: &mut i64, rs1: &i64, rs2: &i64) {
    *rd = ((*rs1 as i32) / (*rs2 as i32)) as i64;
}

pub(super) fn divuw(_: &Hart, rd: &mut i64, rs1: &i64, rs2: &i64) {
    *rd = ((*rs1 as u32) / (*rs2 as u32)) as i32 as i64;
}

pub(super) fn remw(_: &Hart, rd: &mut i64, rs1: &i64, rs2: &i64) {
    *rd = ((*rs1 as i32) % (*rs2 as i32)) as i64;
}

pub(super) fn remuw(_: &Hart, rd: &mut i64, rs1: &i64, rs2: &i64) {
    *rd = ((*rs1 as u32) % (*rs2 as u32)) as i32 as i64;
}
