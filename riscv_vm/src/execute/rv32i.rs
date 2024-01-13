use crate::{
    hart::Hart,
    memory::{address::Address, Memory},
};

use super::{ExecuteError, ExecuteResult};

pub(super) fn lui(_: &Hart, rd: &mut i64, imm: i32) -> Result<ExecuteResult, ExecuteError> {
    *rd = imm.into();
    Ok(ExecuteResult::Continue)
}

pub(super) fn auipc(hart: &Hart, rd: &mut i64, imm: i32) -> Result<ExecuteResult, ExecuteError> {
    *rd = (hart.get_pc() + imm).into();
    Ok(ExecuteResult::Continue)
}

pub(super) fn jal(hart: &mut Hart, rd: &mut i64, imm: i32) -> Result<ExecuteResult, ExecuteError> {
    let imm = (imm << 11) >> 11;
    *rd = hart.get_pc().into();
    *rd += 4;
    Ok(ExecuteResult::Jump(hart.get_pc() + imm))
}

pub(super) fn jalr(
    hart: &mut Hart,
    rd: &mut i64,
    rs1: &i64,
    imm: i32,
) -> Result<ExecuteResult, ExecuteError> {
    let imm = (imm << 20) >> 20;
    *rd = hart.get_pc().into();
    *rd += 4;
    Ok(ExecuteResult::Jump(
        (rs1.wrapping_add(imm.into()) & !1).into(),
    ))
}

pub(super) fn beq(
    hart: &mut Hart,
    rs1: &i64,
    rs2: &i64,
    imm: i32,
) -> Result<ExecuteResult, ExecuteError> {
    let imm = ((imm << 19) >> 19);
    if (rs1 == rs2) {
        Ok(ExecuteResult::Jump(hart.get_pc() + imm))
    } else {
        Ok(ExecuteResult::Continue)
    }
}

pub(super) fn bne(
    hart: &mut Hart,
    rs1: &i64,
    rs2: &i64,
    imm: i32,
) -> Result<ExecuteResult, ExecuteError> {
    let imm = ((imm << 19) >> 19);
    if (rs1 != rs2) {
        Ok(ExecuteResult::Jump(hart.get_pc() + imm))
    } else {
        Ok(ExecuteResult::Continue)
    }
}

pub(super) fn blt(
    hart: &mut Hart,
    rs1: &i64,
    rs2: &i64,
    imm: i32,
) -> Result<ExecuteResult, ExecuteError> {
    let imm = ((imm << 19) >> 19);
    if rs1 < rs2 {
        Ok(ExecuteResult::Jump(hart.get_pc() + imm))
    } else {
        Ok(ExecuteResult::Continue)
    }
}

pub(super) fn bge(
    hart: &mut Hart,
    rs1: &i64,
    rs2: &i64,
    imm: i32,
) -> Result<ExecuteResult, ExecuteError> {
    let imm = ((imm << 19) >> 19);
    if rs1 >= rs2 {
        Ok(ExecuteResult::Jump(hart.get_pc() + imm))
    } else {
        Ok(ExecuteResult::Continue)
    }
}

pub(super) fn bltu(
    hart: &mut Hart,
    rs1: &i64,
    rs2: &i64,
    imm: i32,
) -> Result<ExecuteResult, ExecuteError> {
    let imm = ((imm << 19) >> 19);
    if (*rs1 as u64) < (*rs2 as u64) {
        Ok(ExecuteResult::Jump(hart.get_pc() + imm))
    } else {
        Ok(ExecuteResult::Continue)
    }
}

pub(super) fn bgeu(
    hart: &mut Hart,
    rs1: &i64,
    rs2: &i64,
    imm: i32,
) -> Result<ExecuteResult, ExecuteError> {
    let imm = ((imm << 19) >> 19);
    if (*rs1 as u64) >= (*rs2 as u64) {
        Ok(ExecuteResult::Jump(hart.get_pc() + imm))
    } else {
        Ok(ExecuteResult::Continue)
    }
}

pub(super) fn lb<const SIZE: usize>(
    _: &Hart,
    mem: &mut Memory<SIZE>,
    rd: &mut i64,
    rs1: &i64,
    imm: i32,
) -> Result<ExecuteResult, ExecuteError> {
    let imm = (imm << 20) >> 20;
    let bytes = mem.read_bytes(rs1.overflowing_add(imm.into()).0.into(), 1)?;
    let mut buf = [0; 1];
    buf.copy_from_slice(&bytes);
    *rd = i8::from_le_bytes(buf) as i64;
    Ok(ExecuteResult::Continue)
}

pub(super) fn lh<const SIZE: usize>(
    _: &Hart,
    mem: &mut Memory<SIZE>,
    rd: &mut i64,
    rs1: &i64,
    imm: i32,
) -> Result<ExecuteResult, ExecuteError> {
    let imm = (imm << 20) >> 20;
    let bytes = mem.read_bytes(rs1.overflowing_add(imm.into()).0.into(), 2)?;
    let mut buf = [0; 2];
    buf.copy_from_slice(&bytes);
    *rd = i16::from_le_bytes(buf) as i64;
    Ok(ExecuteResult::Continue)
}

pub(super) fn lw<const SIZE: usize>(
    _: &Hart,
    mem: &mut Memory<SIZE>,
    rd: &mut i64,
    rs1: &i64,
    imm: i32,
) -> Result<ExecuteResult, ExecuteError> {
    let imm = (imm << 20) >> 20;
    let bytes = mem.read_bytes(rs1.overflowing_add(imm.into()).0.into(), 4)?;
    let mut buf = [0; 4];
    buf.copy_from_slice(&bytes);
    *rd = i32::from_le_bytes(buf) as i64;
    Ok(ExecuteResult::Continue)
}

pub(super) fn lbu<const SIZE: usize>(
    _: &Hart,
    mem: &mut Memory<SIZE>,
    rd: &mut i64,
    rs1: &i64,
    imm: i32,
) -> Result<ExecuteResult, ExecuteError> {
    let imm = (imm << 20) >> 20;
    let bytes = mem.read_bytes(rs1.overflowing_add(imm.into()).0.into(), 1)?;
    let mut buf = [0; 1];
    buf.copy_from_slice(&bytes);
    *rd = u8::from_le_bytes(buf) as i64;
    Ok(ExecuteResult::Continue)
}

pub(super) fn lhu<const SIZE: usize>(
    _: &Hart,
    mem: &mut Memory<SIZE>,
    rd: &mut i64,
    rs1: &i64,
    imm: i32,
) -> Result<ExecuteResult, ExecuteError> {
    let imm = (imm << 20) >> 20;
    let bytes = mem.read_bytes(rs1.overflowing_add(imm.into()).0.into(), 2)?;
    let mut buf = [0; 2];
    buf.copy_from_slice(&bytes);
    *rd = u16::from_le_bytes(buf) as i64;
    Ok(ExecuteResult::Continue)
}

pub(super) fn sb<const SIZE: usize>(
    _: &Hart,
    mem: &mut Memory<SIZE>,
    rs1: &i64,
    rs2: &i64,
    imm: i32,
) -> Result<ExecuteResult, ExecuteError> {
    let imm = (imm << 20) >> 20;
    mem.write_bytes(
        &rs2.to_le_bytes()[0..1],
        rs1.overflowing_add(imm.into()).0.into(),
    )?;
    Ok(ExecuteResult::Continue)
}

pub(super) fn sh<const SIZE: usize>(
    _: &Hart,
    mem: &mut Memory<SIZE>,
    rs1: &i64,
    rs2: &i64,
    imm: i32,
) -> Result<ExecuteResult, ExecuteError> {
    let imm = (imm << 20) >> 20;
    mem.write_bytes(
        &rs2.to_le_bytes()[0..2],
        rs1.overflowing_add(imm.into()).0.into(),
    )?;
    Ok(ExecuteResult::Continue)
}

pub(super) fn sw<const SIZE: usize>(
    _: &Hart,
    mem: &mut Memory<SIZE>,
    rs1: &i64,
    rs2: &i64,
    imm: i32,
) -> Result<ExecuteResult, ExecuteError> {
    let imm = (imm << 20) >> 20;
    mem.write_bytes(
        &rs2.to_le_bytes()[0..4],
        rs1.overflowing_add(imm.into()).0.into(),
    )?;
    Ok(ExecuteResult::Continue)
}

pub(super) fn addi(
    _: &Hart,
    rd: &mut i64,
    rs1: &i64,
    imm: i32,
) -> Result<ExecuteResult, ExecuteError> {
    let imm = (imm << 20) >> 20;
    *rd = rs1.overflowing_add(imm.into()).0;
    Ok(ExecuteResult::Continue)
}

pub(super) fn slti(
    _: &Hart,
    rd: &mut i64,
    rs1: &i64,
    imm: i32,
) -> Result<ExecuteResult, ExecuteError> {
    let imm = (imm << 20) >> 20;
    if *rs1 < (imm as i64) {
        *rd = 1;
    } else {
        *rd = 0;
    }
    Ok(ExecuteResult::Continue)
}

pub(super) fn sltiu(
    _: &Hart,
    rd: &mut i64,
    rs1: &i64,
    imm: i32,
) -> Result<ExecuteResult, ExecuteError> {
    let imm = (imm << 20) >> 20;
    if (*rs1 as u64) < (imm as i64 as u64) {
        *rd = 1;
    } else {
        *rd = 0;
    }
    Ok(ExecuteResult::Continue)
}

pub(super) fn xori(
    _: &Hart,
    rd: &mut i64,
    rs1: &i64,
    imm: i32,
) -> Result<ExecuteResult, ExecuteError> {
    let imm = (imm << 20) >> 20;
    *rd = *rs1 ^ (imm as i64);
    Ok(ExecuteResult::Continue)
}

pub(super) fn ori(
    _: &Hart,
    rd: &mut i64,
    rs1: &i64,
    imm: i32,
) -> Result<ExecuteResult, ExecuteError> {
    let imm = (imm << 20) >> 20;
    *rd = *rs1 | (imm as i64);
    Ok(ExecuteResult::Continue)
}

pub(super) fn andi(
    _: &Hart,
    rd: &mut i64,
    rs1: &i64,
    imm: i32,
) -> Result<ExecuteResult, ExecuteError> {
    let imm = (imm << 20) >> 20;
    *rd = *rs1 & (imm as i64);
    Ok(ExecuteResult::Continue)
}

pub(super) fn slli(
    _: &Hart,
    rd: &mut i64,
    rs1: &i64,
    shamt: i32,
) -> Result<ExecuteResult, ExecuteError> {
    *rd = *rs1 << shamt;
    Ok(ExecuteResult::Continue)
}

pub(super) fn srli(
    _: &Hart,
    rd: &mut i64,
    rs1: &i64,
    shamt: i32,
) -> Result<ExecuteResult, ExecuteError> {
    *rd = ((*rs1 as u64) >> shamt) as i64;
    Ok(ExecuteResult::Continue)
}

pub(super) fn srai(
    _: &Hart,
    rd: &mut i64,
    rs1: &i64,
    imm: i32,
) -> Result<ExecuteResult, ExecuteError> {
    *rd = *rs1 >> imm;
    Ok(ExecuteResult::Continue)
}

pub(super) fn add(
    _: &Hart,
    rd: &mut i64,
    rs1: &i64,
    rs2: &i64,
) -> Result<ExecuteResult, ExecuteError> {
    *rd = rs1.overflowing_add(*rs2).0;
    Ok(ExecuteResult::Continue)
}

pub(super) fn sub(
    _: &Hart,
    rd: &mut i64,
    rs1: &i64,
    rs2: &i64,
) -> Result<ExecuteResult, ExecuteError> {
    *rd = rs1.overflowing_add(*rs2).0;
    Ok(ExecuteResult::Continue)
}

pub(super) fn sll(
    _: &Hart,
    rd: &mut i64,
    rs1: &i64,
    rs2: &i64,
) -> Result<ExecuteResult, ExecuteError> {
    *rd = rs1 << (rs2 & 0b111111);
    Ok(ExecuteResult::Continue)
}

pub(super) fn slt(
    _: &Hart,
    rd: &mut i64,
    rs1: &i64,
    rs2: &i64,
) -> Result<ExecuteResult, ExecuteError> {
    if *rs1 < *rs2 {
        *rd = 1;
    } else {
        *rd = 0;
    }
    Ok(ExecuteResult::Continue)
}

pub(super) fn sltu(
    _: &Hart,
    rd: &mut i64,
    rs1: &i64,
    rs2: &i64,
) -> Result<ExecuteResult, ExecuteError> {
    if (*rs1 as u64) < (*rs2 as u64) {
        *rd = 1;
    } else {
        *rd = 0;
    }
    Ok(ExecuteResult::Continue)
}

pub(super) fn xor(
    _: &Hart,
    rd: &mut i64,
    rs1: &i64,
    rs2: &i64,
) -> Result<ExecuteResult, ExecuteError> {
    *rd = rs1 ^ rs2;
    Ok(ExecuteResult::Continue)
}

pub(super) fn srl(
    _: &Hart,
    rd: &mut i64,
    rs1: &i64,
    rs2: &i64,
) -> Result<ExecuteResult, ExecuteError> {
    *rd = ((*rs1 as u64) >> (rs2 & 0b111111)) as i64;
    Ok(ExecuteResult::Continue)
}

pub(super) fn sra(
    _: &Hart,
    rd: &mut i64,
    rs1: &i64,
    rs2: &i64,
) -> Result<ExecuteResult, ExecuteError> {
    *rd = *rs1 >> (rs2 & 0b111111);
    Ok(ExecuteResult::Continue)
}

pub(super) fn or(
    _: &Hart,
    rd: &mut i64,
    rs1: &i64,
    rs2: &i64,
) -> Result<ExecuteResult, ExecuteError> {
    *rd = rs1 | rs2;
    Ok(ExecuteResult::Continue)
}

pub(super) fn and(
    _: &Hart,
    rd: &mut i64,
    rs1: &i64,
    rs2: &i64,
) -> Result<ExecuteResult, ExecuteError> {
    *rd = rs1 & rs2;
    Ok(ExecuteResult::Continue)
}
