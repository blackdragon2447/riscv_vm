// Most of these warnings are caused ny macro generated code and cannot be avoided.
#![allow(clippy::unnecessary_cast)]

mod rv32a;
mod rv32i;
mod rv32m;
mod rv64a;
mod rv64i;
mod rv64m;
mod rv64zicsr;

use enumflags2::BitFlags;

use crate::{
    decode::instruction::{Instruction, Instruction::*},
    hart::{
        isa::Isa, privilege::PrivilegeMode, registers::IntRegister, trap::Exception, CsrAddress,
        Hart,
    },
    memory::{address::Address, Memory, MemoryError, MemoryWindow},
};

pub enum ExecuteResult {
    Continue,
    Jump(Address),
    CsrUpdate(CsrAddress),
}

#[derive(Debug)]
pub enum ExecuteError {
    Exception(Exception),
    Fatal,
}

impl From<MemoryError> for ExecuteError {
    fn from(value: MemoryError) -> Self {
        match value {
            MemoryError::OutOfBoundsWrite(_) => Self::Exception(Exception::StoreAccessFault),
            MemoryError::OutOfBoundsRead(_) => Self::Exception(Exception::LoadAccessFault),
            MemoryError::OutOfMemory => Self::Exception(Exception::StoreAccessFault),
            MemoryError::PmpDeniedWrite => Self::Exception(Exception::StoreAccessFault),
            MemoryError::PmpDeniedRead => Self::Exception(Exception::LoadAccessFault),
            MemoryError::PageFaultRead => Self::Exception(Exception::LoadPageFault),
            MemoryError::PageFaultWrite => Self::Exception(Exception::StorePageFault),
            MemoryError::DeviceMemoryPoison => Self::Fatal,
            MemoryError::PmpDeniedFetch => Self::Exception(Exception::InstructionAccessFault),
            MemoryError::PageFaultFetch => Self::Exception(Exception::InstructionPageFault),
        }
    }
}

pub fn execute_rv64(
    hart: &mut Hart,
    mem: &mut Memory,
    instruction: Instruction,
    isa: BitFlags<Isa>,
) -> Result<ExecuteResult, ExecuteError> {
    match instruction {
        // rv64i
        LUI { rd, imm } => u_type(hart, rd, imm, rv32i::lui_64),
        AUIPC { rd, imm } => u_type(hart, rd, imm, rv32i::auipc_64),
        JAL { rd, imm } => u_type(hart, rd, imm, rv32i::jal_64),
        JALR { rd, rs1, imm } => i_type(hart, rd, rs1, imm, rv32i::jalr_64),
        BEQ { rs1, rs2, imm } => s_type(hart, imm, rs1, rs2, rv32i::beq_64),
        BNE { rs1, rs2, imm } => s_type(hart, imm, rs1, rs2, rv32i::bne_64),
        BLT { rs1, rs2, imm } => s_type(hart, imm, rs1, rs2, rv32i::blt_64),
        BGE { rs1, rs2, imm } => s_type(hart, imm, rs1, rs2, rv32i::bge_64),
        BLTU { rs1, rs2, imm } => s_type(hart, imm as i32, rs1, rs2, rv32i::bltu_64),
        BGEU { rs1, rs2, imm } => s_type(hart, imm as i32, rs1, rs2, rv32i::bgeu_64),
        LB { rd, rs1, imm } => i_type_mem(hart, mem, rd, rs1, imm, rv32i::lb_64),
        LH { rd, rs1, imm } => i_type_mem(hart, mem, rd, rs1, imm, rv32i::lh_64),
        LW { rd, rs1, imm } => i_type_mem(hart, mem, rd, rs1, imm, rv32i::lw_64),
        LD { rd, rs1, imm } => i_type_mem(hart, mem, rd, rs1, imm, rv64i::ld_64),
        LBU { rd, rs1, imm } => i_type_mem(hart, mem, rd, rs1, imm, rv32i::lbu_64),
        LHU { rd, rs1, imm } => i_type_mem(hart, mem, rd, rs1, imm, rv32i::lhu_64),
        LWU { rd, rs1, imm } => i_type_mem(hart, mem, rd, rs1, imm, rv64i::lwu_64),
        SB { rs1, rs2, imm } => s_type_mem(hart, mem, imm, rs1, rs2, rv32i::sb_64),
        SH { rs1, rs2, imm } => s_type_mem(hart, mem, imm, rs1, rs2, rv32i::sh_64),
        SW { rs1, rs2, imm } => s_type_mem(hart, mem, imm, rs1, rs2, rv32i::sw_64),
        SD { rs1, rs2, imm } => s_type_mem(hart, mem, imm, rs1, rs2, rv64i::sd_64),
        ADDI { rd, rs1, imm } => i_type(hart, rd, rs1, imm, rv32i::addi_64),
        SLTI { rd, rs1, imm } => i_type(hart, rd, rs1, imm, rv32i::slti_64),
        SLTIU { rd, rs1, imm } => i_type(hart, rd, rs1, imm as i32, rv32i::sltiu_64),
        XORI { rd, rs1, imm } => i_type(hart, rd, rs1, imm, rv32i::xori_64),
        ORI { rd, rs1, imm } => i_type(hart, rd, rs1, imm, rv32i::ori_64),
        ANDI { rd, rs1, imm } => i_type(hart, rd, rs1, imm, rv32i::andi_64),
        SLLI { rd, rs1, shamt } => i_type(hart, rd, rs1, shamt, rv32i::slli_64),
        SRLI { rd, rs1, shamt } => i_type(hart, rd, rs1, shamt, rv32i::srli_64),
        SRAI { rd, rs1, shamt } => i_type(hart, rd, rs1, shamt, rv32i::srai_64),
        ADD { rd, rs1, rs2 } => r_type(hart, rd, rs1, rs2, rv32i::add_64),
        SUB { rd, rs1, rs2 } => r_type(hart, rd, rs1, rs2, rv32i::sub_64),
        SLL { rd, rs1, rs2 } => r_type(hart, rd, rs1, rs2, rv32i::sll_64),
        SLT { rd, rs1, rs2 } => r_type(hart, rd, rs1, rs2, rv32i::slt_64),
        SLTU { rd, rs1, rs2 } => r_type(hart, rd, rs1, rs2, rv32i::sltu_64),
        XOR { rd, rs1, rs2 } => r_type(hart, rd, rs1, rs2, rv32i::xor_64),
        SRL { rd, rs1, rs2 } => r_type(hart, rd, rs1, rs2, rv32i::srl_64),
        SRA { rd, rs1, rs2 } => r_type(hart, rd, rs1, rs2, rv32i::sra_64),
        OR { rd, rs1, rs2 } => r_type(hart, rd, rs1, rs2, rv32i::or_64),
        AND { rd, rs1, rs2 } => r_type(hart, rd, rs1, rs2, rv32i::and_64),
        ADDIW { rd, rs1, imm } => i_type(hart, rd, rs1, imm, rv64i::addiw_64),
        SLTIW { rd, rs1, imm } => i_type(hart, rd, rs1, imm, rv64i::sltiw_64),
        SLLIW { rd, rs1, shamt } => i_type(hart, rd, rs1, shamt, rv64i::slliw_64),
        SRLIW { rd, rs1, shamt } => i_type(hart, rd, rs1, shamt, rv64i::srliw_64),
        SRAIW { rd, rs1, shamt } => i_type(hart, rd, rs1, shamt, rv64i::sraiw_64),
        ADDW { rd, rs1, rs2 } => r_type(hart, rd, rs1, rs2, rv64i::addw_64),
        SUBW { rd, rs1, rs2 } => r_type(hart, rd, rs1, rs2, rv64i::subw_64),
        SLLW { rd, rs1, rs2 } => r_type(hart, rd, rs1, rs2, rv64i::sllw_64),
        SLTW { rd, rs1, rs2 } => r_type(hart, rd, rs1, rs2, rv64i::sltw_64),
        SRLW { rd, rs1, rs2 } => r_type(hart, rd, rs1, rs2, rv64i::srlw_64),
        SRAW { rd, rs1, rs2 } => r_type(hart, rd, rs1, rs2, rv64i::sraw_64),

        // rv64m
        MUL { rd, rs1, rs2 } if isa.contains(Isa::M) => r_type(hart, rd, rs1, rs2, rv32m::mul_64),
        MULH { rd, rs1, rs2 } if isa.contains(Isa::M) => r_type(hart, rd, rs1, rs2, rv32m::mulh_64),
        MULHSU { rd, rs1, rs2 } if isa.contains(Isa::M) => {
            r_type(hart, rd, rs1, rs2, rv32m::mulhsu_64)
        }
        MULHU { rd, rs1, rs2 } if isa.contains(Isa::M) => {
            r_type(hart, rd, rs1, rs2, rv32m::mulhu_64)
        }
        DIV { rd, rs1, rs2 } if isa.contains(Isa::M) => r_type(hart, rd, rs1, rs2, rv32m::div_64),
        DIVU { rd, rs1, rs2 } if isa.contains(Isa::M) => r_type(hart, rd, rs1, rs2, rv32m::divu_64),
        REM { rd, rs1, rs2 } if isa.contains(Isa::M) => r_type(hart, rd, rs1, rs2, rv32m::rem_64),
        REMU { rd, rs1, rs2 } if isa.contains(Isa::M) => r_type(hart, rd, rs1, rs2, rv32m::remu_64),
        MULW { rd, rs1, rs2 } if isa.contains(Isa::M) => r_type(hart, rd, rs1, rs2, rv64m::mulw_64),
        DIVW { rd, rs1, rs2 } if isa.contains(Isa::M) => r_type(hart, rd, rs1, rs2, rv64m::divw_64),
        DIVUW { rd, rs1, rs2 } if isa.contains(Isa::M) => {
            r_type(hart, rd, rs1, rs2, rv64m::divuw_64)
        }
        REMW { rd, rs1, rs2 } if isa.contains(Isa::M) => r_type(hart, rd, rs1, rs2, rv64m::remw_64),
        REMUW { rd, rs1, rs2 } if isa.contains(Isa::M) => {
            r_type(hart, rd, rs1, rs2, rv64m::remuw_64)
        }

        // rv32a
        LR_W { rd, rs1, rl, aq } => r_type_mem(hart, mem, rd, rs1, IntRegister::X0, rv32a::lr_w_64),
        SC_W {
            rd,
            rs1,
            rs2,
            rl,
            aq,
        } => r_type_mem(hart, mem, rd, rs1, rs2, rv32a::sc_w_64),
        AMOSWAP_W {
            rd,
            rs1,
            rs2,
            rl,
            aq,
        } => r_type_mem(hart, mem, rd, rs1, rs2, rv32a::amoswap_w_64),
        AMOADD_W {
            rd,
            rs1,
            rs2,
            rl,
            aq,
        } => r_type_mem(hart, mem, rd, rs1, rs2, rv32a::amoadd_w_64),
        AMOAND_W {
            rd,
            rs1,
            rs2,
            rl,
            aq,
        } => r_type_mem(hart, mem, rd, rs1, rs2, rv32a::amoand_w_64),
        AMOOR_W {
            rd,
            rs1,
            rs2,
            rl,
            aq,
        } => r_type_mem(hart, mem, rd, rs1, rs2, rv32a::amoor_w_64),
        AMOXOR_W {
            rd,
            rs1,
            rs2,
            rl,
            aq,
        } => r_type_mem(hart, mem, rd, rs1, rs2, rv32a::amoxor_w_64),
        AMOMAX_W {
            rd,
            rs1,
            rs2,
            rl,
            aq,
        } => r_type_mem(hart, mem, rd, rs1, rs2, rv32a::amomax_w_64),
        AMOMAXU_W {
            rd,
            rs1,
            rs2,
            rl,
            aq,
        } => r_type_mem(hart, mem, rd, rs1, rs2, rv32a::amomaxu_w_64),
        AMOMIN_W {
            rd,
            rs1,
            rs2,
            rl,
            aq,
        } => r_type_mem(hart, mem, rd, rs1, rs2, rv32a::amomin_w_64),
        AMOMINU_W {
            rd,
            rs1,
            rs2,
            rl,
            aq,
        } => r_type_mem(hart, mem, rd, rs1, rs2, rv32a::amominu_w_64),

        // rv64a
        LR_D { rd, rs1, rl, aq } => r_type_mem(hart, mem, rd, rs1, IntRegister::X0, rv64a::lr_d_64),
        SC_D {
            rd,
            rs1,
            rs2,
            rl,
            aq,
        } => r_type_mem(hart, mem, rd, rs1, rs2, rv64a::sc_d_64),
        AMOSWAP_D {
            rd,
            rs1,
            rs2,
            rl,
            aq,
        } => r_type_mem(hart, mem, rd, rs1, rs2, rv64a::amoswap_d_64),
        AMOADD_D {
            rd,
            rs1,
            rs2,
            rl,
            aq,
        } => r_type_mem(hart, mem, rd, rs1, rs2, rv64a::amoadd_d_64),
        AMOAND_D {
            rd,
            rs1,
            rs2,
            rl,
            aq,
        } => r_type_mem(hart, mem, rd, rs1, rs2, rv64a::amoand_d_64),
        AMOOR_D {
            rd,
            rs1,
            rs2,
            rl,
            aq,
        } => r_type_mem(hart, mem, rd, rs1, rs2, rv64a::amoor_d_64),
        AMOXOR_D {
            rd,
            rs1,
            rs2,
            rl,
            aq,
        } => r_type_mem(hart, mem, rd, rs1, rs2, rv64a::amoxor_d_64),
        AMOMAX_D {
            rd,
            rs1,
            rs2,
            rl,
            aq,
        } => r_type_mem(hart, mem, rd, rs1, rs2, rv64a::amomax_d_64),
        AMOMAXU_D {
            rd,
            rs1,
            rs2,
            rl,
            aq,
        } => r_type_mem(hart, mem, rd, rs1, rs2, rv64a::amomaxu_d_64),
        AMOMIN_D {
            rd,
            rs1,
            rs2,
            rl,
            aq,
        } => r_type_mem(hart, mem, rd, rs1, rs2, rv64a::amomin_d_64),
        AMOMINU_D {
            rd,
            rs1,
            rs2,
            rl,
            aq,
        } => r_type_mem(hart, mem, rd, rs1, rs2, rv64a::amominu_d_64),

        // rv64 Zicsr
        CSRRW {
            rd: IntRegister::X0,
            rs1,
            csr,
        } => inst_csrwo(hart, rs1, csr, rv64zicsr::csrw),
        CSRRW { rd, rs1, csr } => inst_csr(hart, rd, rs1, csr, rv64zicsr::csrrw),
        CSRRS {
            rd,
            rs1: IntRegister::X0,
            csr,
        } => inst_csrro(hart, rd, csr, rv64zicsr::csrr),
        CSRRS { rd, rs1, csr } => inst_csr(hart, rd, rs1, csr, rv64zicsr::csrrs),
        CSRRC {
            rd,
            rs1: IntRegister::X0,
            csr,
        } => inst_csrro(hart, rd, csr, rv64zicsr::csrr),
        CSRRC { rd, rs1, csr } => inst_csr(hart, rd, rs1, csr, rv64zicsr::csrrc),
        CSRRWI {
            rd: IntRegister::X0,
            uimm,
            csr,
        } => inst_csrwoi(hart, uimm, csr, rv64zicsr::csrwi),
        CSRRWI { rd, uimm, csr } => inst_csri(hart, rd, uimm, csr, rv64zicsr::csrrwi),
        CSRRSI { rd, uimm: 0, csr } => inst_csrroi(hart, rd, csr, rv64zicsr::csrr),
        CSRRSI { rd, uimm, csr } => inst_csri(hart, rd, uimm, csr, rv64zicsr::csrrsi),
        CSRRCI { rd, uimm: 0, csr } => inst_csrroi(hart, rd, csr, rv64zicsr::csrr),
        CSRRCI { rd, uimm, csr } => inst_csri(hart, rd, uimm, csr, rv64zicsr::csrrci),

        // privileged
        MRET if hart.privilege() >= PrivilegeMode::Machine => {
            let status = hart.get_csr_mut().get_status_mut();
            let mpp = status.mpp;
            status.mie = status.mpie;
            status.mpie = false;
            if mpp != PrivilegeMode::Machine {
                status.mprv = false;
            }
            hart.set_privilege(mpp);

            Ok(ExecuteResult::Jump(hart.get_csr().get_mepc()))
        }

        SRET if hart.get_csr().get_status().tsr
            && hart.privilege() == PrivilegeMode::Supervisor =>
        {
            Err(ExecuteError::Exception(Exception::IllegalInstruction))
        }
        SRET if hart.privilege() >= PrivilegeMode::Supervisor => {
            let status = hart.get_csr_mut().get_status_mut();
            let spp = status.spp;
            status.sie = status.spie;
            status.spie = false;
            if spp != PrivilegeMode::Machine {
                status.mprv = false;
            }
            hart.set_privilege(spp);

            Ok(ExecuteResult::Jump(hart.get_csr().get_sepc()))
        }

        // ???
        FENCE { rd, rs1, imm } => nop(),
        ECALL => match hart.privilege() {
            PrivilegeMode::User => Err(ExecuteError::Exception(Exception::EcallUMode)),
            PrivilegeMode::Supervisor => Err(ExecuteError::Exception(Exception::EcallSMode)),
            PrivilegeMode::Machine => Err(ExecuteError::Exception(Exception::EcallMMode)),
        },
        EBREAK => Err(ExecuteError::Exception(Exception::BreakPoint)),

        WFI if hart.get_csr().get_status().tw && hart.privilege() < PrivilegeMode::Machine => {
            // Timeout is 0
            Err(ExecuteError::Exception(Exception::IllegalInstruction))
        }
        WFI => Err(ExecuteError::Exception(Exception::IllegalInstruction)), // Not implemented

        // SFENCE.VLA if !hart.get_csr().status.tvm
        // SINVAL.VLA if !hart.get_csr().status.tvm
        //
        Undifined(i) => {
            eprintln!("Trying to execute Undifined Instruction: {:#8x}", i);
            Err(ExecuteError::Exception(Exception::IllegalInstruction))
        }
        _ => Err(ExecuteError::Exception(Exception::IllegalInstruction)),
    }
}

fn r_type<E>(
    hart: &mut Hart,
    rd: IntRegister,
    rs1: IntRegister,
    rs2: IntRegister,
    executor: E,
) -> Result<ExecuteResult, ExecuteError>
where
    E: Fn(Address, &mut i64, &i64, &i64) -> Result<ExecuteResult, ExecuteError>,
{
    let rs1 = hart.get_reg(rs1);
    let rs2 = hart.get_reg(rs2);
    let mut rdv = 0;
    let result = executor(hart.get_pc(), &mut rdv, &rs1, &rs2)?;
    hart.set_reg(rd, rdv);
    Ok(result)
}

fn r_type_mem<E>(
    hart: &mut Hart,
    mem: &mut Memory,
    rd: IntRegister,
    rs1: IntRegister,
    rs2: IntRegister,
    executor: E,
) -> Result<ExecuteResult, ExecuteError>
where
    E: Fn(Address, MemoryWindow, &mut i64, &i64, &i64) -> Result<ExecuteResult, ExecuteError>,
{
    let rs1 = hart.get_reg(rs1);
    let rs2 = hart.get_reg(rs2);
    let mut rdv = 0;
    let result = executor(hart.get_pc(), mem.window(hart), &mut rdv, &rs1, &rs2)?;
    hart.set_reg(rd, rdv);
    Ok(result)
}

fn i_type<E>(
    hart: &mut Hart,
    rd: IntRegister,
    rs1: IntRegister,
    imm: i32,
    executor: E,
) -> Result<ExecuteResult, ExecuteError>
where
    E: Fn(Address, &mut i64, &i64, i32) -> Result<ExecuteResult, ExecuteError>,
{
    let rs1 = hart.get_reg(rs1);
    let mut rdv = 0;
    let result = executor(hart.get_pc(), &mut rdv, &rs1, imm)?;
    hart.set_reg(rd, rdv);
    Ok(result)
}

fn i_type_mem<E>(
    hart: &mut Hart,
    mem: &mut Memory,
    rd: IntRegister,
    rs1: IntRegister,
    imm: i32,
    executor: E,
) -> Result<ExecuteResult, ExecuteError>
where
    E: Fn(Address, MemoryWindow, &mut i64, &i64, i32) -> Result<ExecuteResult, ExecuteError>,
{
    let rs1 = hart.get_reg(rs1);
    let mut rdv = 0;
    let result = executor(hart.get_pc(), mem.window(hart), &mut rdv, &rs1, imm)?;
    hart.set_reg(rd, rdv);
    Ok(result)
}

fn s_type<E>(
    hart: &mut Hart,
    imm: i32,
    rs1: IntRegister,
    rs2: IntRegister,
    executor: E,
) -> Result<ExecuteResult, ExecuteError>
where
    E: Fn(Address, &i64, &i64, i32) -> Result<ExecuteResult, ExecuteError>,
{
    let rs1 = hart.get_reg(rs1);
    let rs2 = hart.get_reg(rs2);
    let result = executor(hart.get_pc(), &rs1, &rs2, imm)?;
    Ok(result)
}

fn s_type_mem<E>(
    hart: &mut Hart,
    mem: &mut Memory,
    imm: i32,
    rs1: IntRegister,
    rs2: IntRegister,
    executor: E,
) -> Result<ExecuteResult, ExecuteError>
where
    E: Fn(Address, MemoryWindow, &i64, &i64, i32) -> Result<ExecuteResult, ExecuteError>,
{
    let rs1 = hart.get_reg(rs1);
    let rs2 = hart.get_reg(rs2);
    let result = executor(hart.get_pc(), mem.window(hart), &rs1, &rs2, imm)?;
    Ok(result)
}

fn u_type<E>(
    hart: &mut Hart,
    rd: IntRegister,
    imm: i32,
    executor: E,
) -> Result<ExecuteResult, ExecuteError>
where
    E: Fn(Address, &mut i64, i32) -> Result<ExecuteResult, ExecuteError>,
{
    let mut rdv = 0;
    let result = executor(hart.get_pc(), &mut rdv, imm)?;
    hart.set_reg(rd, rdv);
    Ok(result)
}

fn inst_csr<E>(
    hart: &mut Hart,
    rd: IntRegister,
    rs1: IntRegister,
    csr: CsrAddress,
    executor: E,
) -> Result<ExecuteResult, ExecuteError>
where
    E: Fn(&mut Hart, &mut i64, &i64, CsrAddress) -> Result<ExecuteResult, ExecuteError>,
{
    let mut rdv = 0;
    let rs1 = hart.get_reg(rs1);
    let result = executor(hart, &mut rdv, &rs1, csr)?;
    hart.set_reg(rd, rdv);
    Ok(result)
}

fn inst_csrwo<E>(
    hart: &mut Hart,
    rs1: IntRegister,
    csr: CsrAddress,
    executor: E,
) -> Result<ExecuteResult, ExecuteError>
where
    E: Fn(&mut Hart, &i64, CsrAddress) -> Result<ExecuteResult, ExecuteError>,
{
    let rs1 = hart.get_reg(rs1);
    executor(hart, &rs1, csr)
}

fn inst_csrro<E>(
    hart: &mut Hart,
    rd: IntRegister,
    csr: CsrAddress,
    executor: E,
) -> Result<ExecuteResult, ExecuteError>
where
    E: Fn(&mut Hart, &mut i64, CsrAddress) -> Result<ExecuteResult, ExecuteError>,
{
    let mut rdv = 0;
    let result = executor(hart, &mut rdv, csr)?;
    hart.set_reg(rd, rdv);
    Ok(result)
}

fn inst_csri<E>(
    hart: &mut Hart,
    rd: IntRegister,
    uimm: u32,
    csr: CsrAddress,
    executor: E,
) -> Result<ExecuteResult, ExecuteError>
where
    E: Fn(&mut Hart, &mut i64, u32, CsrAddress) -> Result<ExecuteResult, ExecuteError>,
{
    let mut rdv = 0;
    let result = executor(hart, &mut rdv, uimm, csr)?;
    hart.set_reg(rd, rdv);
    Ok(result)
}

fn inst_csrwoi<E>(
    hart: &mut Hart,
    imm: u32,
    csr: CsrAddress,
    executor: E,
) -> Result<ExecuteResult, ExecuteError>
where
    E: Fn(&mut Hart, u32, CsrAddress) -> Result<ExecuteResult, ExecuteError>,
{
    executor(hart, imm, csr)
}

fn inst_csrroi<E>(
    hart: &mut Hart,
    rd: IntRegister,
    csr: CsrAddress,
    executor: E,
) -> Result<ExecuteResult, ExecuteError>
where
    E: Fn(&mut Hart, &mut i64, CsrAddress) -> Result<ExecuteResult, ExecuteError>,
{
    let mut rdv = 0;
    let resut = executor(hart, &mut rdv, csr)?;
    hart.set_reg(rd, rdv);
    Ok(resut)
}

fn nop() -> Result<ExecuteResult, ExecuteError> {
    Ok(ExecuteResult::Continue)
}
