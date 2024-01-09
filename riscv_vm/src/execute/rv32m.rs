use crate::{
    hart::Hart,
    memory::{address::Address, Memory},
};

pub(super) fn mul(_: &Hart, rd: &mut i64, rs1: &i64, rs2: &i64) {
    *rd = rs1.overflowing_mul(*rs2).0;
}

pub(super) fn mulh(_: &Hart, rd: &mut i64, rs1: &i64, rs2: &i64) {
    *rd = ((*rs1 as i128).overflowing_mul((*rs2 as i128)).0 >> 64) as i64;
}

pub(super) fn mulhsu(_: &Hart, rd: &mut i64, rs1: &i64, rs2: &i64) {
    *rd = ((*rs1 as i128).overflowing_mul((*rs2 as u128 as i128)).0 >> 64) as i64;
}

pub(super) fn mulhu(_: &Hart, rd: &mut i64, rs1: &i64, rs2: &i64) {
    *rd = ((*rs1 as u128).overflowing_mul((*rs2 as u128)).0 >> 64) as i64;
}

pub(super) fn div(_: &Hart, rd: &mut i64, rs1: &i64, rs2: &i64) {
    *rd = rs1 / rs2;
}

pub(super) fn divu(_: &Hart, rd: &mut i64, rs1: &i64, rs2: &i64) {
    *rd = ((*rs1 as u64) / (*rs2 as u64)) as i64;
}

pub(super) fn rem(_: &Hart, rd: &mut i64, rs1: &i64, rs2: &i64) {
    *rd = rs1 % rs2;
}

pub(super) fn remu(_: &Hart, rd: &mut i64, rs1: &i64, rs2: &i64) {
    *rd = ((*rs1 as u64) % (*rs2 as u64)) as i64;
}
