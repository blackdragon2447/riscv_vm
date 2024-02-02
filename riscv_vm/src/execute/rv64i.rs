use riscv_vm_macros::inst;

use crate::memory::{address::Address, MemoryWindow};

use super::{ExecuteError, ExecuteResult};

inst!(ld(i_mem) for [64]: {
    let bytes = mem.read_bytes(rs1.overflowing_add(imm.into()).0.into(), 8)?;
    let mut buf = [0; 8];
    buf.copy_from_slice(&bytes);
    *rd = i64::from_le_bytes(buf);
    Ok(ExecuteResult::Continue)
});

inst!(lwu(i_mem) for [64]: {
    let bytes = mem.read_bytes(rs1.overflowing_add(imm.into()).0.into(), 4)?;
    let mut buf = [0; 4];
    buf.copy_from_slice(&bytes);
    *rd = u32::from_le_bytes(buf) as ixlen;
    Ok(ExecuteResult::Continue)
});

inst!(sd(s_mem) for [64]: {
    mem.write_bytes(
        &rs2.to_le_bytes()[0..8],
        rs1.overflowing_add(imm as i64).0.into(),
    )?;
    Ok(ExecuteResult::Continue)
});

inst!(addiw(i) for [64]: {
    *rd = (*rs1 as i32).overflowing_add(imm).0 as ixlen;
    Ok(ExecuteResult::Continue)
});

inst!(sltiw(i) for [64]: {
    if (*rs1 as i32) < imm {
        *rd = 1;
    } else {
        *rd = 0;
    }
    Ok(ExecuteResult::Continue)
});

inst!(slliw(i) for [64]: {
    *rd = ((*rs1 as u32) << imm) as i32 as ixlen;
    Ok(ExecuteResult::Continue)
});

inst!(srliw(i) for [64]: {
    *rd = ((*rs1 as u32) >> imm) as i32 as ixlen;
    Ok(ExecuteResult::Continue)
});

inst!(sraiw(i) for [64]: {
    *rd = ((*rs1 as i32) >> imm) as uxlen as ixlen;
    Ok(ExecuteResult::Continue)
});

inst!(addw(r) for [64]: {
    *rd = (*rs1 as i32).overflowing_add(*rs2 as i32).0 as ixlen;
    Ok(ExecuteResult::Continue)
});

inst!(subw(r) for [64]: {
    *rd = (*rs1 as i32).overflowing_sub(*rs2 as i32).0 as ixlen;
    Ok(ExecuteResult::Continue)
});

inst!(sllw(r) for [64]: {
    *rd = ((*rs1 as u32) << (rs2 & 0b11111)) as i32 as ixlen;
    Ok(ExecuteResult::Continue)
});

inst!(sltw(r) for [64]: {
    if (*rs1 as i32) < (*rs2 as i32) {
        *rd = 1;
    } else {
        *rd = 0;
    }
    Ok(ExecuteResult::Continue)
});

inst!(srlw(r) for [64]: {
    *rd = ((*rs1 as u32) >> (rs2 & 0b11111)) as i32 as ixlen;
    Ok(ExecuteResult::Continue)
});

inst!(sraw(r) for [64]: {
    *rd = (*rs1 as i32).overflowing_shr((rs2 & 0b111111) as u32).0 as ixlen;
    Ok(ExecuteResult::Continue)
});
