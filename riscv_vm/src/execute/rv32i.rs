use crate::memory::{address::Address, Memory};

use super::{ExecuteError, ExecuteResult};

pub(super) fn lui(_: Address, rd: &mut i64, imm: i32) -> Result<ExecuteResult, ExecuteError> {
    *rd = imm as i64;
    Ok(ExecuteResult::Continue)
}

pub(super) fn auipc(pc: Address, rd: &mut i64, imm: i32) -> Result<ExecuteResult, ExecuteError> {
    *rd = (pc + imm).into();
    Ok(ExecuteResult::Continue)
}

pub(super) fn jal(pc: Address, rd: &mut i64, imm: i32) -> Result<ExecuteResult, ExecuteError> {
    let imm = (imm << 11) >> 11;
    *rd = pc.into();
    *rd += 4;
    Ok(ExecuteResult::Jump(pc + imm))
}

pub(super) fn jalr(
    pc: Address,
    rd: &mut i64,
    rs1: &i64,
    imm: i32,
) -> Result<ExecuteResult, ExecuteError> {
    let imm = (imm << 20) >> 20;
    *rd = pc.into();
    *rd += 4;
    Ok(ExecuteResult::Jump(
        (rs1.wrapping_add(imm.into()) & !1).into(),
    ))
}

pub(super) fn beq(
    pc: Address,
    rs1: &i64,
    rs2: &i64,
    imm: i32,
) -> Result<ExecuteResult, ExecuteError> {
    let imm = ((imm << 19) >> 19);
    if (rs1 == rs2) {
        Ok(ExecuteResult::Jump(pc + imm))
    } else {
        Ok(ExecuteResult::Continue)
    }
}

pub(super) fn bne(
    pc: Address,
    rs1: &i64,
    rs2: &i64,
    imm: i32,
) -> Result<ExecuteResult, ExecuteError> {
    let imm = ((imm << 19) >> 19);
    if (rs1 != rs2) {
        Ok(ExecuteResult::Jump(pc + imm))
    } else {
        Ok(ExecuteResult::Continue)
    }
}

pub(super) fn blt(
    pc: Address,
    rs1: &i64,
    rs2: &i64,
    imm: i32,
) -> Result<ExecuteResult, ExecuteError> {
    let imm = ((imm << 19) >> 19);
    if rs1 < rs2 {
        Ok(ExecuteResult::Jump(pc + imm))
    } else {
        Ok(ExecuteResult::Continue)
    }
}

pub(super) fn bge(
    pc: Address,
    rs1: &i64,
    rs2: &i64,
    imm: i32,
) -> Result<ExecuteResult, ExecuteError> {
    let imm = ((imm << 19) >> 19);
    if rs1 >= rs2 {
        Ok(ExecuteResult::Jump(pc + imm))
    } else {
        Ok(ExecuteResult::Continue)
    }
}

pub(super) fn bltu(
    pc: Address,
    rs1: &i64,
    rs2: &i64,
    imm: i32,
) -> Result<ExecuteResult, ExecuteError> {
    let imm = ((imm << 19) >> 19);
    if (*rs1 as u64) < (*rs2 as u64) {
        Ok(ExecuteResult::Jump(pc + imm))
    } else {
        Ok(ExecuteResult::Continue)
    }
}

pub(super) fn bgeu(
    pc: Address,
    rs1: &i64,
    rs2: &i64,
    imm: i32,
) -> Result<ExecuteResult, ExecuteError> {
    let imm = ((imm << 19) >> 19);
    if (*rs1 as u64) >= (*rs2 as u64) {
        Ok(ExecuteResult::Jump(pc + imm))
    } else {
        Ok(ExecuteResult::Continue)
    }
}

pub(super) fn lb<const SIZE: usize>(
    _: Address,
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
    _: Address,
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
    _: Address,
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
    _: Address,
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
    _: Address,
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
    _: Address,
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
    _: Address,
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
    _: Address,
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
    _: Address,
    rd: &mut i64,
    rs1: &i64,
    imm: i32,
) -> Result<ExecuteResult, ExecuteError> {
    let imm = (imm << 20) >> 20;
    *rd = rs1.overflowing_add(imm.into()).0;
    Ok(ExecuteResult::Continue)
}

pub(super) fn slti(
    _: Address,
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
    _: Address,
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
    _: Address,
    rd: &mut i64,
    rs1: &i64,
    imm: i32,
) -> Result<ExecuteResult, ExecuteError> {
    let imm = (imm << 20) >> 20;
    *rd = *rs1 ^ (imm as i64);
    Ok(ExecuteResult::Continue)
}

pub(super) fn ori(
    _: Address,
    rd: &mut i64,
    rs1: &i64,
    imm: i32,
) -> Result<ExecuteResult, ExecuteError> {
    let imm = (imm << 20) >> 20;
    *rd = *rs1 | (imm as i64);
    Ok(ExecuteResult::Continue)
}

pub(super) fn andi(
    _: Address,
    rd: &mut i64,
    rs1: &i64,
    imm: i32,
) -> Result<ExecuteResult, ExecuteError> {
    let imm = (imm << 20) >> 20;
    *rd = *rs1 & (imm as i64);
    Ok(ExecuteResult::Continue)
}

pub(super) fn slli(
    _: Address,
    rd: &mut i64,
    rs1: &i64,
    shamt: i32,
) -> Result<ExecuteResult, ExecuteError> {
    *rd = *rs1 << shamt;
    Ok(ExecuteResult::Continue)
}

pub(super) fn srli(
    _: Address,
    rd: &mut i64,
    rs1: &i64,
    shamt: i32,
) -> Result<ExecuteResult, ExecuteError> {
    *rd = ((*rs1 as u64) >> shamt) as i64;
    Ok(ExecuteResult::Continue)
}

pub(super) fn srai(
    _: Address,
    rd: &mut i64,
    rs1: &i64,
    imm: i32,
) -> Result<ExecuteResult, ExecuteError> {
    *rd = *rs1 >> imm;
    Ok(ExecuteResult::Continue)
}

pub(super) fn add(
    _: Address,
    rd: &mut i64,
    rs1: &i64,
    rs2: &i64,
) -> Result<ExecuteResult, ExecuteError> {
    *rd = rs1.overflowing_add(*rs2).0;
    Ok(ExecuteResult::Continue)
}

pub(super) fn sub(
    _: Address,
    rd: &mut i64,
    rs1: &i64,
    rs2: &i64,
) -> Result<ExecuteResult, ExecuteError> {
    *rd = rs1.overflowing_sub(*rs2).0;
    Ok(ExecuteResult::Continue)
}

pub(super) fn sll(
    _: Address,
    rd: &mut i64,
    rs1: &i64,
    rs2: &i64,
) -> Result<ExecuteResult, ExecuteError> {
    *rd = rs1 << (rs2 & 0b111111);
    Ok(ExecuteResult::Continue)
}

pub(super) fn slt(
    _: Address,
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
    _: Address,
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
    _: Address,
    rd: &mut i64,
    rs1: &i64,
    rs2: &i64,
) -> Result<ExecuteResult, ExecuteError> {
    *rd = rs1 ^ rs2;
    Ok(ExecuteResult::Continue)
}

pub(super) fn srl(
    _: Address,
    rd: &mut i64,
    rs1: &i64,
    rs2: &i64,
) -> Result<ExecuteResult, ExecuteError> {
    *rd = ((*rs1 as u64) >> (rs2 & 0b111111)) as i64;
    Ok(ExecuteResult::Continue)
}

pub(super) fn sra(
    _: Address,
    rd: &mut i64,
    rs1: &i64,
    rs2: &i64,
) -> Result<ExecuteResult, ExecuteError> {
    *rd = *rs1 >> (rs2 & 0b111111);
    Ok(ExecuteResult::Continue)
}

pub(super) fn or(
    _: Address,
    rd: &mut i64,
    rs1: &i64,
    rs2: &i64,
) -> Result<ExecuteResult, ExecuteError> {
    *rd = rs1 | rs2;
    Ok(ExecuteResult::Continue)
}

pub(super) fn and(
    _: Address,
    rd: &mut i64,
    rs1: &i64,
    rs2: &i64,
) -> Result<ExecuteResult, ExecuteError> {
    *rd = rs1 & rs2;
    Ok(ExecuteResult::Continue)
}
