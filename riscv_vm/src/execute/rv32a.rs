#![allow(clippy::useless_conversion)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

use std::cmp::{max, min};

use riscv_vm_macros::inst;

use crate::{
    hart::trap::Exception,
    memory::{address::Address, Memory, MemoryWindow},
};

use super::{ExecuteError, ExecuteResult};

inst!(lr_w(r_mem) for [b32, b64]
    where [rd: int, rs1: int, rs2: int]:
{
    if *rs1 % 4 != 0 {
        return Err(ExecuteError::Exception(Exception::LoadAddressMisaligned));
    }
    let bytes = mem.read_reserve((*rs1).into(), 4)?;
    *rd = i32::from_le_bytes(bytes.try_into().unwrap()) as ixlen;
    Ok(ExecuteResult::Continue)
});

inst!(sc_w(r_mem) for [b32, b64]
    where [rd: int, rs1: int, rs2: int]:
{
    if *rs1 % 4 != 0 {
        return Err(ExecuteError::Exception(Exception::LoadAddressMisaligned));
    }
    // invert because riscv wants us to write 1 on a fail (return false) where in rust false is
    // 0x00
    *rd = !mem.write_conditional(&(*rs2 as i32).to_le_bytes(), (*rs1).into())? as ixlen;
    Ok(ExecuteResult::Continue)
});

inst!(amoswap_w(r_mem) for [b32, b64]
    where [rd: int, rs1: int, rs2: int]:
{
    if *rs1 % 4 != 0 {
        return Err(ExecuteError::Exception(Exception::LoadAddressMisaligned));
    }
    *rd = mem.atomic_operation_w((*rs1).into(), *rs2 as i32, |orig, _| orig)? as ixlen;
    Ok(ExecuteResult::Continue)
});

inst!(amoadd_w(r_mem) for [b32, b64]
    where [rd: int, rs1: int, rs2: int]:
{
    if *rs1 % 4 != 0 {
        return Err(ExecuteError::Exception(Exception::LoadAddressMisaligned));
    }
    *rd = mem.atomic_operation_w((*rs1).into(), *rs2 as i32, |orig, rs| orig.overflowing_add(rs).0)? as ixlen;
    Ok(ExecuteResult::Continue)
});

inst!(amoand_w(r_mem) for [b32, b64]
    where [rd: int, rs1: int, rs2: int]:
{
    if *rs1 % 4 != 0 {
        return Err(ExecuteError::Exception(Exception::LoadAddressMisaligned));
    }
    *rd = mem.atomic_operation_w((*rs1).into(), *rs2 as i32, |orig, rs| orig & rs)? as ixlen;
    Ok(ExecuteResult::Continue)
});

inst!(amoor_w(r_mem) for [b32, b64]
    where [rd: int, rs1: int, rs2: int]:
{
    if *rs1 % 4 != 0 {
        return Err(ExecuteError::Exception(Exception::LoadAddressMisaligned));
    }
    *rd = mem.atomic_operation_w((*rs1).into(), *rs2 as i32, |orig, rs| orig | rs)? as ixlen;
    Ok(ExecuteResult::Continue)
});

inst!(amoxor_w(r_mem) for [b32, b64]
    where [rd: int, rs1: int, rs2: int]:
{
    if *rs1 % 4 != 0 {
        return Err(ExecuteError::Exception(Exception::LoadAddressMisaligned));
    }
    *rd = mem.atomic_operation_w((*rs1).into(), *rs2 as i32, |orig, rs| orig ^ rs)? as ixlen;
    Ok(ExecuteResult::Continue)
});

inst!(amomax_w(r_mem) for [b32, b64]
    where [rd: int, rs1: int, rs2: int]:
{
    if *rs1 % 4 != 0 {
        return Err(ExecuteError::Exception(Exception::LoadAddressMisaligned));
    }
    *rd = mem.atomic_operation_w((*rs1).into(), *rs2 as i32, max)? as ixlen;
    Ok(ExecuteResult::Continue)
});

inst!(amomaxu_w(r_mem) for [b32, b64]
    where [rd: int, rs1: int, rs2: int]:
{
    if *rs1 % 4 != 0 {
        return Err(ExecuteError::Exception(Exception::LoadAddressMisaligned));
    }
    *rd = mem.atomic_operation_w((*rs1).into(), *rs2 as i32, |orig, rs| max(orig as u32, rs as u32) as i32)? as ixlen;
    Ok(ExecuteResult::Continue)
});

inst!(amomin_w(r_mem) for [b32, b64]
    where [rd: int, rs1: int, rs2: int]:
{
    if *rs1 % 4 != 0 {
        return Err(ExecuteError::Exception(Exception::LoadAddressMisaligned));
    }
    *rd = mem.atomic_operation_w((*rs1).into(), *rs2 as i32, min)? as ixlen;
    Ok(ExecuteResult::Continue)
});

inst!(amominu_w(r_mem) for [b32, b64]
    where [rd: int, rs1: int, rs2: int]:
{
    if *rs1 % 4 != 0 {
        return Err(ExecuteError::Exception(Exception::LoadAddressMisaligned));
    }
    *rd = mem.atomic_operation_w((*rs1).into(), *rs2 as i32, |orig, rs| min(orig as u32, rs as u32) as i32)? as ixlen;
    Ok(ExecuteResult::Continue)
});
