use crate::{hart::Hart, memory::Memory};

pub(super) fn ld<const SIZE: usize>(
    _: &Hart,
    mem: &mut Memory<SIZE>,
    rd: &mut i64,
    rs1: &i64,
    imm: i32,
) {
    let imm = (imm << 20) >> 20;
    let bytes = mem
        .read_bytes(rs1.overflowing_add(imm.into()).0.into(), 8)
        .unwrap();
    let mut buf = [0; 8];
    buf.copy_from_slice(bytes);
    *rd = i64::from_le_bytes(buf);
}

pub(super) fn lwu<const SIZE: usize>(
    _: &Hart,
    mem: &mut Memory<SIZE>,
    rd: &mut i64,
    rs1: &i64,
    imm: i32,
) {
    let imm = (imm << 20) >> 20;
    let bytes = mem
        .read_bytes(rs1.overflowing_add(imm.into()).0.into(), 4)
        .unwrap();
    let mut buf = [0; 4];
    buf.copy_from_slice(bytes);
    *rd = u32::from_le_bytes(buf) as i64;
}

pub(super) fn sd<const SIZE: usize>(
    _: &Hart,
    mem: &mut Memory<SIZE>,
    rs1: &i64,
    rs2: &i64,
    imm: i32,
) {
    let imm = (imm << 20) >> 20;
    mem.write_bytes(
        &rs2.to_le_bytes()[0..8],
        rs1.overflowing_add(imm.into()).0.into(),
    )
    .unwrap();
}

pub(super) fn addiw(_: &Hart, rd: &mut i64, rs1: &i64, imm: i32) {
    let imm = (imm << 20) >> 20;
    *rd = (*rs1 as i32).overflowing_add(imm).0 as i64;
}

pub(super) fn sltiw(_: &Hart, rd: &mut i64, rs1: &i64, imm: i32) {
    let imm = (imm << 20) >> 20;
    if (*rs1 as i32) < imm {
        *rd = 1;
    } else {
        *rd = 0;
    }
}

pub(super) fn slliw(_: &Hart, rd: &mut i64, rs1: &i64, shamt: i32) {
    *rd = ((*rs1 as u32) << shamt) as i64;
}

pub(super) fn srliw(_: &Hart, rd: &mut i64, rs1: &i64, imm: i32) {
    *rd = ((*rs1 as u32) >> imm) as i64;
}

pub(super) fn sraiw(_: &Hart, rd: &mut i64, rs1: &i64, imm: i32) {
    *rd = ((*rs1 as i32) >> imm) as i64;
}

pub(super) fn addw(_: &Hart, rd: &mut i64, rs1: &i64, rs2: &i64) {
    *rd = (*rs1 as i32).overflowing_add(*rs2 as i32).0 as i64;
}

pub(super) fn subw(_: &Hart, rd: &mut i64, rs1: &i64, rs2: &i64) {
    *rd = (*rs1 as i32).saturating_sub(*rs2 as i32) as i64;
}

pub(super) fn sllw(_: &Hart, rd: &mut i64, rs1: &i64, rs2: &i64) {
    *rd = ((*rs1 as u32) << (rs2 & 0b11111)) as i32 as i64;
}

pub(super) fn sltw(_: &Hart, rd: &mut i64, rs1: &i64, rs2: &i64) {
    if (*rs1 as i32) < (*rs2 as i32) {
        *rd = 1;
    } else {
        *rd = 0;
    }
}

pub(super) fn srlw(_: &Hart, rd: &mut i64, rs1: &i64, rs2: &i64) {
    *rd = ((*rs1 as u32) >> (rs2 & 0b11111)) as i32 as i64;
}

pub(super) fn sraw(_: &Hart, rd: &mut i64, rs1: &i64, rs2: &i64) {
    *rd = ((*rs1 as i32) >> (rs2 & 0b111111)) as i64;
}
