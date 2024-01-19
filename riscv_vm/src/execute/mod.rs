mod rv32i;
mod rv32m;
mod rv64i;
mod rv64m;
mod rv64zicsr;

use enumflags2::BitFlags;

use crate::{
    decode::instruction::{Instruction, Instruction::*},
    hart::{isa::Isa, privilege::PrivilegeMode, trap::Exception, CsrAddress, Hart},
    memory::{address::Address, registers::IntRegister, Memory, MemoryError},
};

pub enum ExecuteResult {
    Continue,
    Jump(Address),
}

#[derive(Debug)]
pub enum ExecuteError {
    Exception(Exception),
    Fatal,
}

impl From<MemoryError> for ExecuteError {
    fn from(value: MemoryError) -> Self {
        match value {
            MemoryError::OutOfBoundsWrite(_, _) => Self::Exception(Exception::StoreAccessFault),
            MemoryError::OutOfBoundsRead(_, _) => Self::Exception(Exception::LoadAccessFault),
            MemoryError::OutOfMemory => Self::Exception(Exception::StoreAccessFault),
            MemoryError::DeviceMemoryPoison => Self::Fatal,
        }
    }
}

pub fn execute<const SIZE: usize>(
    hart: &mut Hart,
    mem: &mut Memory<SIZE>,
    instruction: Instruction,
    isa: BitFlags<Isa>,
) -> Result<ExecuteResult, ExecuteError> {
    match instruction {
        // rv64i
        LUI { rd, imm } => u_type(hart, rd, imm, rv32i::lui),
        AUIPC { rd, imm } => u_type(hart, rd, imm, rv32i::auipc),
        JAL { rd, imm } => u_type(hart, rd, imm, rv32i::jal),
        JALR { rd, rs1, imm } => i_type(hart, rd, rs1, imm, rv32i::jalr),
        BEQ { rs1, rs2, imm } => s_type(hart, imm, rs1, rs2, rv32i::beq),
        BNE { rs1, rs2, imm } => s_type(hart, imm, rs1, rs2, rv32i::bne),
        BLT { rs1, rs2, imm } => s_type(hart, imm, rs1, rs2, rv32i::blt),
        BGE { rs1, rs2, imm } => s_type(hart, imm, rs1, rs2, rv32i::bge),
        BLTU { rs1, rs2, imm } => s_type(hart, imm as i32, rs1, rs2, rv32i::bltu),
        BGEU { rs1, rs2, imm } => s_type(hart, imm as i32, rs1, rs2, rv32i::bgeu),
        LB { rd, rs1, imm } => i_type_mem_access(hart, mem, rd, rs1, imm, rv32i::lb),
        LH { rd, rs1, imm } => i_type_mem_access(hart, mem, rd, rs1, imm, rv32i::lh),
        LW { rd, rs1, imm } => i_type_mem_access(hart, mem, rd, rs1, imm, rv32i::lw),
        LD { rd, rs1, imm } => i_type_mem_access(hart, mem, rd, rs1, imm, rv64i::ld),
        LBU { rd, rs1, imm } => i_type_mem_access(hart, mem, rd, rs1, imm, rv32i::lbu),
        LHU { rd, rs1, imm } => i_type_mem_access(hart, mem, rd, rs1, imm, rv32i::lhu),
        LWU { rd, rs1, imm } => i_type_mem_access(hart, mem, rd, rs1, imm, rv64i::lwu),
        SB { rs1, rs2, imm } => s_type_mem_access(hart, mem, imm, rs1, rs2, rv32i::sb),
        SH { rs1, rs2, imm } => s_type_mem_access(hart, mem, imm, rs1, rs2, rv32i::sh),
        SW { rs1, rs2, imm } => s_type_mem_access(hart, mem, imm, rs1, rs2, rv32i::sw),
        SD { rs1, rs2, imm } => s_type_mem_access(hart, mem, imm, rs1, rs2, rv64i::sd),
        ADDI { rd, rs1, imm } => i_type(hart, rd, rs1, imm, rv32i::addi),
        SLTI { rd, rs1, imm } => i_type(hart, rd, rs1, imm, rv32i::slti),
        SLTIU { rd, rs1, imm } => i_type(hart, rd, rs1, imm as i32, rv32i::sltiu),
        XORI { rd, rs1, imm } => i_type(hart, rd, rs1, imm, rv32i::xori),
        ORI { rd, rs1, imm } => i_type(hart, rd, rs1, imm, rv32i::ori),
        ANDI { rd, rs1, imm } => i_type(hart, rd, rs1, imm, rv32i::andi),
        SLLI { rd, rs1, shamt } => i_type_shift(hart, rd, rs1, shamt, rv32i::slli),
        SRLI { rd, rs1, shamt } => i_type_shift(hart, rd, rs1, shamt, rv32i::srli),
        SRAI { rd, rs1, shamt } => i_type_shift(hart, rd, rs1, shamt, rv32i::srai),
        ADD { rd, rs1, rs2 } => r_type(hart, rd, rs1, rs2, rv32i::add),
        SUB { rd, rs1, rs2 } => r_type(hart, rd, rs1, rs2, rv32i::sub),
        SLL { rd, rs1, rs2 } => r_type(hart, rd, rs1, rs2, rv32i::sll),
        SLT { rd, rs1, rs2 } => r_type(hart, rd, rs1, rs2, rv32i::slt),
        SLTU { rd, rs1, rs2 } => r_type(hart, rd, rs1, rs2, rv32i::sltu),
        XOR { rd, rs1, rs2 } => r_type(hart, rd, rs1, rs2, rv32i::xor),
        SRL { rd, rs1, rs2 } => r_type(hart, rd, rs1, rs2, rv32i::srl),
        SRA { rd, rs1, rs2 } => r_type(hart, rd, rs1, rs2, rv32i::sra),
        OR { rd, rs1, rs2 } => r_type(hart, rd, rs1, rs2, rv32i::or),
        AND { rd, rs1, rs2 } => r_type(hart, rd, rs1, rs2, rv32i::and),
        ADDIW { rd, rs1, imm } => i_type(hart, rd, rs1, imm, rv64i::addiw),
        SLTIW { rd, rs1, imm } => i_type(hart, rd, rs1, imm, rv64i::sltiw),
        SLLIW { rd, rs1, shamt } => i_type_shift(hart, rd, rs1, shamt, rv64i::slliw),
        SRLIW { rd, rs1, shamt } => i_type_shift(hart, rd, rs1, shamt, rv64i::srliw),
        SRAIW { rd, rs1, shamt } => i_type_shift(hart, rd, rs1, shamt, rv64i::sraiw),
        ADDW { rd, rs1, rs2 } => r_type(hart, rd, rs1, rs2, rv64i::addw),
        SUBW { rd, rs1, rs2 } => r_type(hart, rd, rs1, rs2, rv64i::subw),
        SLLW { rd, rs1, rs2 } => r_type(hart, rd, rs1, rs2, rv64i::sllw),
        SLTW { rd, rs1, rs2 } => r_type(hart, rd, rs1, rs2, rv64i::sltw),
        SRLW { rd, rs1, rs2 } => r_type(hart, rd, rs1, rs2, rv64i::srlw),
        SRAW { rd, rs1, rs2 } => r_type(hart, rd, rs1, rs2, rv64i::sraw),

        // rv64m
        MUL { rd, rs1, rs2 } if isa.contains(Isa::M) => r_type(hart, rd, rs1, rs2, rv32m::mul),
        MULH { rd, rs1, rs2 } if isa.contains(Isa::M) => r_type(hart, rd, rs1, rs2, rv32m::mulh),
        MULHSU { rd, rs1, rs2 } if isa.contains(Isa::M) => {
            r_type(hart, rd, rs1, rs2, rv32m::mulhsu)
        }
        MULHU { rd, rs1, rs2 } if isa.contains(Isa::M) => r_type(hart, rd, rs1, rs2, rv32m::mulhu),
        DIV { rd, rs1, rs2 } if isa.contains(Isa::M) => r_type(hart, rd, rs1, rs2, rv32m::div),
        DIVU { rd, rs1, rs2 } if isa.contains(Isa::M) => r_type(hart, rd, rs1, rs2, rv32m::divu),
        REM { rd, rs1, rs2 } if isa.contains(Isa::M) => r_type(hart, rd, rs1, rs2, rv32m::rem),
        REMU { rd, rs1, rs2 } if isa.contains(Isa::M) => r_type(hart, rd, rs1, rs2, rv32m::remu),
        MULW { rd, rs1, rs2 } if isa.contains(Isa::M) => r_type(hart, rd, rs1, rs2, rv64m::mulw),
        DIVW { rd, rs1, rs2 } if isa.contains(Isa::M) => r_type(hart, rd, rs1, rs2, rv64m::divw),
        DIVUW { rd, rs1, rs2 } if isa.contains(Isa::M) => r_type(hart, rd, rs1, rs2, rv64m::divuw),
        REMW { rd, rs1, rs2 } if isa.contains(Isa::M) => r_type(hart, rd, rs1, rs2, rv64m::remw),
        REMUW { rd, rs1, rs2 } if isa.contains(Isa::M) => r_type(hart, rd, rs1, rs2, rv64m::remuw),

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

        MRET => {
            let status = hart.get_csr().status_mut();
            let mpp = status.mpp;
            status.mie = status.mpie;
            status.mpie = false;
            hart.set_privilege(mpp);

            Ok(ExecuteResult::Jump(hart.get_csr().get_mepc()))
        }

        SRET => {
            let status = hart.get_csr().status_mut();
            let spp = status.spp;
            status.sie = status.spie;
            status.spie = false;
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

fn i_type_shift<E>(
    hart: &mut Hart,
    rd: IntRegister,
    rs1: IntRegister,
    shamt: i32,
    executor: E,
) -> Result<ExecuteResult, ExecuteError>
where
    E: Fn(Address, &mut i64, &i64, i32) -> Result<ExecuteResult, ExecuteError>,
{
    let rs1 = hart.get_reg(rs1);
    let mut rdv = 0;
    let result = executor(hart.get_pc(), &mut rdv, &rs1, shamt)?;
    hart.set_reg(rd, rdv);
    Ok(result)
}

fn i_type_mem_access<E, const SIZE: usize>(
    hart: &mut Hart,
    mem: &mut Memory<SIZE>,
    rd: IntRegister,
    rs1: IntRegister,
    imm: i32,
    executor: E,
) -> Result<ExecuteResult, ExecuteError>
where
    E: Fn(Address, &mut Memory<SIZE>, &mut i64, &i64, i32) -> Result<ExecuteResult, ExecuteError>,
{
    let rs1 = hart.get_reg(rs1);
    let mut rdv = 0;
    let result = executor(hart.get_pc(), mem, &mut rdv, &rs1, imm)?;
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

fn s_type_mem_access<E, const SIZE: usize>(
    hart: &mut Hart,
    mem: &mut Memory<SIZE>,
    imm: i32,
    rs1: IntRegister,
    rs2: IntRegister,
    executor: E,
) -> Result<ExecuteResult, ExecuteError>
where
    E: Fn(Address, &mut Memory<SIZE>, &i64, &i64, i32) -> Result<ExecuteResult, ExecuteError>,
{
    let rs1 = hart.get_reg(rs1);
    let rs2 = hart.get_reg(rs2);
    let result = executor(hart.get_pc(), mem, &rs1, &rs2, imm)?;
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
