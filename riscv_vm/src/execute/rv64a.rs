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

inst!(lr_d(r_mem) for [b64]
    where [rd: int, rs1: int, rs2: int]:
{
    if *rs1 % 4 != 0 {
        return Err(ExecuteError::Exception(Exception::LoadAddressMisaligned));
    }
    let bytes = mem.read_reserve((*rs1).into(), 4)?;
    *rd = i64::from_le_bytes(bytes.try_into().unwrap()) as ixlen;
    Ok(ExecuteResult::Continue)
});

inst!(sc_d(r_mem) for [b64]
    where [rd: int, rs1: int, rs2: int]:
{
    if *rs1 % 4 != 0 {
        return Err(ExecuteError::Exception(Exception::LoadAddressMisaligned));
    }
    // invert because riscv wants us to write 1 on a fail (return false) where in rust false is
    // 0x00
    *rd = !mem.write_conditional(&(*rs2 as i64).to_le_bytes(), (*rs1).into())? as ixlen;
    Ok(ExecuteResult::Continue)

});

inst!(amoswap_d(r_mem) for [b64]
    where [rd: int, rs1: int, rs2: int]:
{
    if *rs1 % 4 != 0 {
        return Err(ExecuteError::Exception(Exception::LoadAddressMisaligned));
    }
    *rd = mem.atomic_operation_d((*rs1).into(), *rs2, |orig, _| orig)? as ixlen;
    Ok(ExecuteResult::Continue)
});

inst!(amoadd_d(r_mem) for [b64]
    where [rd: int, rs1: int, rs2: int]:
{
    if *rs1 % 4 != 0 {
        return Err(ExecuteError::Exception(Exception::LoadAddressMisaligned));
    }
    *rd = mem.atomic_operation_d((*rs1).into(), *rs2 as i64, |orig, rs| orig.overflowing_add(rs).0)? as ixlen;
    Ok(ExecuteResult::Continue)
});

inst!(amoand_d(r_mem) for [b64]
    where [rd: int, rs1: int, rs2: int]:
{
    if *rs1 % 4 != 0 {
        return Err(ExecuteError::Exception(Exception::LoadAddressMisaligned));
    }
    *rd = mem.atomic_operation_d((*rs1).into(), *rs2, |orig, rs| orig & rs)? as ixlen;
    Ok(ExecuteResult::Continue)
});

inst!(amoor_d(r_mem) for [b64]
    where [rd: int, rs1: int, rs2: int]:
{
    if *rs1 % 4 != 0 {
        return Err(ExecuteError::Exception(Exception::LoadAddressMisaligned));
    }
    *rd = mem.atomic_operation_d((*rs1).into(), *rs2, |orig, rs| orig | rs)? as ixlen;
    Ok(ExecuteResult::Continue)
});

inst!(amoxor_d(r_mem) for [b64]
    where [rd: int, rs1: int, rs2: int]:
{
    if *rs1 % 4 != 0 {
        return Err(ExecuteError::Exception(Exception::LoadAddressMisaligned));
    }
    *rd = mem.atomic_operation_d((*rs1).into(), *rs2, |orig, rs| orig ^ rs)? as ixlen;
    Ok(ExecuteResult::Continue)
});

inst!(amomax_d(r_mem) for [b64]
    where [rd: int, rs1: int, rs2: int]:
{
    if *rs1 % 4 != 0 {
        return Err(ExecuteError::Exception(Exception::LoadAddressMisaligned));
    }
    *rd = mem.atomic_operation_d((*rs1).into(), *rs2, max)? as ixlen;
    Ok(ExecuteResult::Continue)
});

inst!(amomaxu_d(r_mem) for [b64]
    where [rd: int, rs1: int, rs2: int]:
{
    if *rs1 % 4 != 0 {
        return Err(ExecuteError::Exception(Exception::LoadAddressMisaligned));
    }
    *rd = mem.atomic_operation_d((*rs1).into(), *rs2, |orig, rs| max(orig as u64, rs as u64) as i64)? as ixlen;
    Ok(ExecuteResult::Continue)
});

inst!(amomin_d(r_mem) for [b64]
    where [rd: int, rs1: int, rs2: int]:
{
    if *rs1 % 4 != 0 {
        return Err(ExecuteError::Exception(Exception::LoadAddressMisaligned));
    }
    *rd = mem.atomic_operation_d((*rs1).into(), *rs2, min)? as ixlen;
    Ok(ExecuteResult::Continue)
});

inst!(amominu_d(r_mem) for [b64]
    where [rd: int, rs1: int, rs2: int]:
{
    if *rs1 % 4 != 0 {
        return Err(ExecuteError::Exception(Exception::LoadAddressMisaligned));
    }
    *rd = mem.atomic_operation_d((*rs1).into(), *rs2, |orig, rs| min(orig as u64, rs as u64) as i64)? as ixlen;
    Ok(ExecuteResult::Continue)
});
