use crate::memory::registers::IntRegister;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Instruction {
    LUI {
        rd: IntRegister,
        imm: u32,
    },
    AUIPC {
        rd: IntRegister,
        imm: u32,
    },

    JAL {
        rd: IntRegister,
        imm: u32,
    },
    JALR {
        rd: IntRegister,
        rs1: IntRegister,
        imm: u32,
    },

    BEQ {
        rs1: IntRegister,
        rs2: IntRegister,
        imm: u32,
    },
    BNE {
        rs1: IntRegister,
        rs2: IntRegister,
        imm: u32,
    },
    BLT {
        rs1: IntRegister,
        rs2: IntRegister,
        imm: u32,
    },
    BGE {
        rs1: IntRegister,
        rs2: IntRegister,
        imm: u32,
    },
    BLTU {
        rs1: IntRegister,
        rs2: IntRegister,
        imm: u32,
    },
    BGEU {
        rs1: IntRegister,
        rs2: IntRegister,
        imm: u32,
    },

    LB {
        rd: IntRegister,
        rs1: IntRegister,
        imm: u32,
    },
    LH {
        rd: IntRegister,
        rs1: IntRegister,
        imm: u32,
    },
    LW {
        rd: IntRegister,
        rs1: IntRegister,
        imm: u32,
    },
    LD {
        rd: IntRegister,
        rs1: IntRegister,
        imm: u32,
    },
    LBU {
        rd: IntRegister,
        rs1: IntRegister,
        imm: u32,
    },
    LHU {
        rd: IntRegister,
        rs1: IntRegister,
        imm: u32,
    },
    LWU {
        rd: IntRegister,
        rs1: IntRegister,
        imm: u32,
    },
    SB {
        rs1: IntRegister,
        rs2: IntRegister,
        imm: u32,
    },
    SH {
        rs1: IntRegister,
        rs2: IntRegister,
        imm: u32,
    },
    SW {
        rs1: IntRegister,
        rs2: IntRegister,
        imm: u32,
    },
    SD {
        rs1: IntRegister,
        rs2: IntRegister,
        imm: u32,
    },

    ADDI {
        rd: IntRegister,
        rs1: IntRegister,
        imm: u32,
    },
    SLTI {
        rd: IntRegister,
        rs1: IntRegister,
        imm: u32,
    },
    SLTIU {
        rd: IntRegister,
        rs1: IntRegister,
        imm: u32,
    },
    XORI {
        rd: IntRegister,
        rs1: IntRegister,
        imm: u32,
    },
    ORI {
        rd: IntRegister,
        rs1: IntRegister,
        imm: u32,
    },
    ANDI {
        rd: IntRegister,
        rs1: IntRegister,
        imm: u32,
    },
    SLLI {
        rd: IntRegister,
        rs1: IntRegister,
        shamt: i32,
    },
    SRLI {
        rd: IntRegister,
        rs1: IntRegister,
        shamt: i32,
    },
    SRAI {
        rd: IntRegister,
        rs1: IntRegister,
        shamt: i32,
    },

    ADD {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
    },
    SUB {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
    },
    SLL {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
    },
    SLT {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
    },
    SLTU {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
    },
    XOR {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
    },
    SRL {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
    },
    SRA {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
    },
    OR {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
    },
    AND {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
    },

    ADDW {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
    },
    SUBW {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
    },
    SLLW {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
    },
    SLTW {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
    },
    SRLW {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
    },
    SRAW {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
    },

    ADDIW {
        rd: IntRegister,
        rs1: IntRegister,
        imm: u32,
    },
    SLTIW {
        rd: IntRegister,
        rs1: IntRegister,
        imm: u32,
    },
    SLLIW {
        rd: IntRegister,
        rs1: IntRegister,
        shamt: i32,
    },
    SRLIW {
        rd: IntRegister,
        rs1: IntRegister,
        shamt: i32,
    },
    SRAIW {
        rd: IntRegister,
        rs1: IntRegister,
        shamt: i32,
    },

    FENCE {
        rd: IntRegister,
        rs1: IntRegister,
        imm: u32,
    },
    ECALL,
    EBREAK,

    Undifined,
}

macro_rules! r_type {
    ($name:ident) => {
        $name {
            rd: Register,
            rs1: Register,
            rs2: Register,
        }
    };
}

macro_rules! I_type {
    ($name:ident) => {
        $name {
            rd: Register,
            rs1: Register,
            imm: u32,
        }
    };
}
