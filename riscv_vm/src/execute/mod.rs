mod rv32i;
mod rv64i;

use crate::{
    decode::instruction::{Instruction, Instruction::*},
    hart::Hart,
    memory::{registers::IntRegister, Memory},
};

pub fn execute<const SIZE: usize>(
    hart: &mut Hart,
    mem: &mut Memory<SIZE>,
    instruction: Instruction,
) {
    match instruction {
        LUI { rd, imm } => u_type(hart, rd, imm, rv32i::lui),
        AUIPC { rd, imm } => u_type(hart, rd, imm, rv32i::auipc),
        JAL { rd, imm } => u_type_mut_hart(hart, rd, imm, rv32i::jal),
        JALR { rd, rs1, imm } => i_type_mut_hart(hart, rd, rs1, imm, rv32i::jalr),
        BEQ { rs1, rs2, imm } => s_type_mut_hart(hart, imm, rs1, rs2, rv32i::beq),
        BNE { rs1, rs2, imm } => s_type_mut_hart(hart, imm, rs1, rs2, rv32i::bne),
        BLT { rs1, rs2, imm } => s_type_mut_hart(hart, imm, rs1, rs2, rv32i::blt),
        BGE { rs1, rs2, imm } => s_type_mut_hart(hart, imm, rs1, rs2, rv32i::bge),
        BLTU { rs1, rs2, imm } => s_type_mut_hart(hart, imm as i32, rs1, rs2, rv32i::bltu),
        BGEU { rs1, rs2, imm } => s_type_mut_hart(hart, imm as i32, rs1, rs2, rv32i::bgeu),
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
        SRLI { rd, rs1, shamt } => i_type_shift(hart, rd, rs1, shamt, rv32i::slli),
        SRAI { rd, rs1, shamt } => i_type_shift(hart, rd, rs1, shamt, rv32i::slli),
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
        FENCE { rd, rs1, imm } => nop(),
        ECALL => nop(),
        EBREAK => nop(),
        Undifined(i) => panic!("Undifined Operation: {:x}", i),
    }
}

fn r_type<E>(hart: &mut Hart, rd: IntRegister, rs1: IntRegister, rs2: IntRegister, executor: E)
where
    E: Fn(&Hart, &mut i64, &i64, &i64),
{
    let rs1 = hart.get_reg(rs1);
    let rs2 = hart.get_reg(rs2);
    let mut rdv = 0;
    executor(&hart, &mut rdv, &rs1, &rs2);
    hart.set_reg(rd, rdv);
    hart.inc_pc();
}

fn i_type<E>(hart: &mut Hart, rd: IntRegister, rs1: IntRegister, imm: i32, executor: E)
where
    E: Fn(&Hart, &mut i64, &i64, i32),
{
    let rs1 = hart.get_reg(rs1);
    let mut rdv = 0;
    executor(&hart, &mut rdv, &rs1, imm);
    hart.set_reg(rd, rdv);
    hart.inc_pc();
}

fn i_type_shift<E>(hart: &mut Hart, rd: IntRegister, rs1: IntRegister, shamt: i32, executor: E)
where
    E: Fn(&Hart, &mut i64, &i64, i32),
{
    let rs1 = hart.get_reg(rs1);
    let mut rdv = 0;
    executor(&hart, &mut rdv, &rs1, shamt);
    hart.set_reg(rd, rdv);
    hart.inc_pc();
}

fn i_type_mem_access<E, const SIZE: usize>(
    hart: &mut Hart,
    mem: &mut Memory<SIZE>,
    rd: IntRegister,
    rs1: IntRegister,
    imm: i32,
    executor: E,
) where
    E: Fn(&Hart, &mut Memory<SIZE>, &mut i64, &i64, i32),
{
    let rs1 = hart.get_reg(rs1);
    let mut rdv = 0;
    executor(&hart, mem, &mut rdv, &rs1, imm);
    hart.set_reg(rd, rdv);
    hart.inc_pc();
}

fn i_type_mut_hart<E>(hart: &mut Hart, rd: IntRegister, rs1: IntRegister, imm: i32, executor: E)
where
    E: Fn(&mut Hart, &mut i64, &i64, i32),
{
    let rs1 = hart.get_reg(rs1);
    let mut rdv = 0;
    executor(hart, &mut rdv, &rs1, imm);
    hart.set_reg(rd, rdv);
}

fn s_type<E>(hart: &mut Hart, imm: i32, rs1: IntRegister, rs2: IntRegister, executor: E)
where
    E: Fn(&Hart, &i64, &i64, i32),
{
    let rs1 = hart.get_reg(rs1);
    let rs2 = hart.get_reg(rs2);
    executor(&hart, &rs1, &rs2, imm);
    hart.inc_pc();
}

fn s_type_mem_access<E, const SIZE: usize>(
    hart: &mut Hart,
    mem: &mut Memory<SIZE>,
    imm: i32,
    rs1: IntRegister,
    rs2: IntRegister,
    executor: E,
) where
    E: Fn(&Hart, &mut Memory<SIZE>, &i64, &i64, i32),
{
    let rs1 = hart.get_reg(rs1);
    let rs2 = hart.get_reg(rs2);
    executor(&hart, mem, &rs1, &rs2, imm);
    hart.inc_pc();
}

fn s_type_mut_hart<E>(hart: &mut Hart, imm: i32, rs1: IntRegister, rs2: IntRegister, executor: E)
where
    E: Fn(&mut Hart, &i64, &i64, i32),
{
    let rs1 = hart.get_reg(rs1);
    let rs2 = hart.get_reg(rs2);
    executor(hart, &rs1, &rs2, imm);
}

fn u_type<E>(hart: &mut Hart, rd: IntRegister, imm: i32, executor: E)
where
    E: Fn(&Hart, &mut i64, i32),
{
    let mut rdv = 0;
    executor(hart, &mut rdv, imm);
    hart.set_reg(rd, rdv);
    hart.inc_pc();
}

fn u_type_mut_hart<E>(hart: &mut Hart, rd: IntRegister, imm: i32, executor: E)
where
    E: Fn(&mut Hart, &mut i64, i32),
{
    let mut rdv = 0;
    executor(hart, &mut rdv, imm);
    hart.set_reg(rd, rdv);
}

fn nop() {}