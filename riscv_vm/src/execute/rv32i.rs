#![allow(clippy::useless_conversion)]

use riscv_vm_macros::inst;

use crate::memory::{address::Address, Memory, MemoryWindow};

use super::{ExecuteError, ExecuteResult};

inst!(lui(u) for [32, 64]: {
    *rd = imm as ixlen;
    Ok(ExecuteResult::Continue)
});

inst!(auipc(u) for [32, 64]: {
    *rd = (pc + imm).into();
    Ok(ExecuteResult::Continue)
});

inst!(jal(u) for [32, 64]: {
    let imm = (imm << 11) >> 11;
    *rd = pc.into();
    *rd += 4;
    Ok(ExecuteResult::Jump(pc + imm))
});

inst!(jalr(i) for [32, 64]: {
    let imm = (imm << 20) >> 20;
    *rd = pc.into();
    *rd += 4;
    Ok(ExecuteResult::Jump(
        (rs1.wrapping_add(imm.into()) & !1).into(),
    ))
});

inst!(beq(s) for [32, 64]: {
    let imm = ((imm << 19) >> 19);
    if (rs1 == rs2) {
        Ok(ExecuteResult::Jump(pc + imm))
    } else {
        Ok(ExecuteResult::Continue)
    }
});

inst!(bne(s) for [32, 64]: {
    let imm = ((imm << 19) >> 19);
    if (rs1 != rs2) {
        Ok(ExecuteResult::Jump(pc + imm))
    } else {
        Ok(ExecuteResult::Continue)
    }
});

inst!(blt(s) for  [32, 64]:{
    let imm = ((imm << 19) >> 19);
    if rs1 < rs2 {
        Ok(ExecuteResult::Jump(pc + imm))
    } else {
        Ok(ExecuteResult::Continue)
    }
});

inst!(bge(s) for [32, 64]: {
    let imm = ((imm << 19) >> 19);
    if rs1 >= rs2 {
        Ok(ExecuteResult::Jump(pc + imm))
    } else {
        Ok(ExecuteResult::Continue)
    }

});

inst!(bltu(s) for [32, 64]: {
    let imm = ((imm << 19) >> 19);
    if (*rs1 as uxlen) < (*rs2 as uxlen) {
        Ok(ExecuteResult::Jump(pc + imm))
    } else {
        Ok(ExecuteResult::Continue)
    }
});

inst!(bgeu(s) for [32, 64]: {
    let imm = ((imm << 19) >> 19);
    if (*rs1 as uxlen) >= (*rs2 as uxlen) {
        Ok(ExecuteResult::Jump(pc + imm))
    } else {
        Ok(ExecuteResult::Continue)
    }
});

inst!(lb(i_mem) for [32, 64]: {
    let imm = (imm << 20) >> 20;
    let bytes = mem.read_bytes(rs1.overflowing_add(imm.into()).0.into(), 1)?;
    let mut buf = [0; 1];
    buf.copy_from_slice(&bytes);
    *rd = i8::from_le_bytes(buf) as ixlen;
    Ok(ExecuteResult::Continue)
});

inst!(lh(i_mem) for [32, 64]: {
    let imm = (imm << 20) >> 20;
    let bytes = mem.read_bytes(rs1.overflowing_add(imm.into()).0.into(), 2)?;
    let mut buf = [0; 2];
    buf.copy_from_slice(&bytes);
    *rd = i16::from_le_bytes(buf) as ixlen;
    Ok(ExecuteResult::Continue)
});

inst!(lw(i_mem) for [32, 64]: {
    let imm = (imm << 20) >> 20;
    let bytes = mem.read_bytes(rs1.overflowing_add(imm.into()).0.into(), 4)?;
    let mut buf = [0; 4];
    buf.copy_from_slice(&bytes);
    *rd = i32::from_le_bytes(buf) as ixlen;
    Ok(ExecuteResult::Continue)
});

inst!(lbu(i_mem) for [32, 64]: {
    let imm = (imm << 20) >> 20;
    let bytes = mem.read_bytes(rs1.overflowing_add(imm.into()).0.into(), 1)?;
    let mut buf = [0; 1];
    buf.copy_from_slice(&bytes);
    *rd = u8::from_le_bytes(buf) as ixlen;
    Ok(ExecuteResult::Continue)
});

inst!(lhu(i_mem) for [32, 64]: {
    let imm = (imm << 20) >> 20;
    let bytes = mem.read_bytes(rs1.overflowing_add(imm.into()).0.into(), 2)?;
    let mut buf = [0; 2];
    buf.copy_from_slice(&bytes);
    *rd = u16::from_le_bytes(buf) as ixlen;
    Ok(ExecuteResult::Continue)
});

inst!(sb(s_mem) for [32, 64]: {
    let imm = (imm << 20) >> 20;
    mem.write_bytes(
        &rs2.to_le_bytes()[0..1],
        rs1.overflowing_add(imm.into()).0.into(),
    )?;
    Ok(ExecuteResult::Continue)
});

inst!(sh(s_mem) for [32, 64]: {
    let imm = (imm << 20) >> 20;
    mem.write_bytes(
        &rs2.to_le_bytes()[0..2],
        rs1.overflowing_add(imm.into()).0.into(),
    )?;
    Ok(ExecuteResult::Continue)
});

inst!(sw(s_mem) for [32, 64]: {
    let imm = (imm << 20) >> 20;
    mem.write_bytes(
        &rs2.to_le_bytes()[0..4],
        rs1.overflowing_add(imm.into()).0.into(),
    )?;
    Ok(ExecuteResult::Continue)
});

inst!(addi(i) for [32, 64]: {
    let imm = (imm << 20) >> 20;
    *rd = rs1.overflowing_add(imm.into()).0;
    Ok(ExecuteResult::Continue)
});

inst!(slti(i) for [32, 64]: {
    let imm = (imm << 20) >> 20;
    if *rs1 < (imm as ixlen) {
        *rd = 1;
    } else {
        *rd = 0;
    }
    Ok(ExecuteResult::Continue)
});

inst!(sltiu(i) for [32, 64]: {
    let imm = (imm << 20) >> 20;
    if (*rs1 as uxlen) < (imm as ixlen as uxlen) {
        *rd = 1;
    } else {
        *rd = 0;
    }
    Ok(ExecuteResult::Continue)
});

inst!(xori(i) for [32, 64]: {
    let imm = (imm << 20) >> 20;
    *rd = *rs1 ^ (imm as ixlen);
    Ok(ExecuteResult::Continue)
});

inst!(ori(i) for [32, 64]: {
    let imm = (imm << 20) >> 20;
    *rd = *rs1 | (imm as ixlen);
    Ok(ExecuteResult::Continue)
});

inst!(andi(i) for [32, 64]: {
    let imm = (imm << 20) >> 20;
    *rd = *rs1 & (imm as ixlen);
    Ok(ExecuteResult::Continue)
});

inst!(slli(i) for [32, 64]: {
    *rd = *rs1 << imm; //shamt
    Ok(ExecuteResult::Continue)
});

inst!(srli(i) for [32, 64]: {
    *rd = ((*rs1 as uxlen) >> imm) as ixlen;
    Ok(ExecuteResult::Continue)
});

inst!(srai(i) for [32, 64]: {
    *rd = *rs1 >> imm;
    Ok(ExecuteResult::Continue)
});

inst!(add(r) for [32, 64]: {
    *rd = rs1.overflowing_add(*rs2).0;
    Ok(ExecuteResult::Continue)
});

inst!(sub(r) for [32, 64]: {
    *rd = rs1.overflowing_sub(*rs2).0;
    Ok(ExecuteResult::Continue)
});

inst!(sll(r) for [32, 64]: {
    *rd = rs1 << (rs2 & 0b111111);
    Ok(ExecuteResult::Continue)
});

inst!(slt(r) for [32, 64]: {
    if *rs1 < *rs2 {
        *rd = 1;
    } else {
        *rd = 0;
    }
    Ok(ExecuteResult::Continue)
});

inst!(sltu(r) for [32, 64]: {
    if (*rs1 as uxlen) < (*rs2 as uxlen) {
        *rd = 1;
    } else {
        *rd = 0;
    }
    Ok(ExecuteResult::Continue)
});

inst!(xor(r) for [32, 64]: {
    *rd = rs1 ^ rs2;
    Ok(ExecuteResult::Continue)
});

inst!(srl(r) for [32, 64]: {
    *rd = ((*rs1 as uxlen) >> (rs2 & 0b111111)) as ixlen;
    Ok(ExecuteResult::Continue)
});

inst!(sra(r) for [32, 64]: {
    *rd = *rs1 >> (rs2 & 0b111111);
    Ok(ExecuteResult::Continue)
});

inst!(or(r) for [32, 64]: {
    *rd = rs1 | rs2;
    Ok(ExecuteResult::Continue)
});

inst!(and(r) for [32, 64]: {
    *rd = rs1 & rs2;
    Ok(ExecuteResult::Continue)
});
