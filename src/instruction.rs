use crate::registers::Register;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Instruction {
    LUI {
        rd: Register,
        imm: u32,
    },
    AUIPC {
        rd: Register,
        imm: u32,
    },

    JAL {
        rd: Register,
        imm: u32,
    },
    JALR {
        rd: Register,
        rs1: Register,
        imm: u32,
    },

    BEQ {
        rs1: Register,
        rs2: Register,
        imm: u32,
    },
    BNE {
        rs1: Register,
        rs2: Register,
        imm: u32,
    },
    BLT {
        rs1: Register,
        rs2: Register,
        imm: u32,
    },
    BGE {
        rs1: Register,
        rs2: Register,
        imm: u32,
    },
    BLTU {
        rs1: Register,
        rs2: Register,
        imm: u32,
    },
    BGEU {
        rs1: Register,
        rs2: Register,
        imm: u32,
    },

    LB {
        rd: Register,
        rs1: Register,
        imm: u32,
    },
    LH {
        rd: Register,
        rs1: Register,
        imm: u32,
    },
    LW {
        rd: Register,
        rs1: Register,
        imm: u32,
    },
    LD {
        rd: Register,
        rs1: Register,
        imm: u32,
    },
    LBU {
        rd: Register,
        rs1: Register,
        imm: u32,
    },
    LHU {
        rd: Register,
        rs1: Register,
        imm: u32,
    },
    LWU {
        rd: Register,
        rs1: Register,
        imm: u32,
    },
    SB {
        rs1: Register,
        rs2: Register,
        imm: u32,
    },
    SH {
        rs1: Register,
        rs2: Register,
        imm: u32,
    },
    SW {
        rs1: Register,
        rs2: Register,
        imm: u32,
    },
    SD {
        rs1: Register,
        rs2: Register,
        imm: u32,
    },

    ADDI {
        rd: Register,
        rs1: Register,
        imm: u32,
    },
    SLTI {
        rd: Register,
        rs1: Register,
        imm: u32,
    },
    SLTIU {
        rd: Register,
        rs1: Register,
        imm: u32,
    },
    XORI {
        rd: Register,
        rs1: Register,
        imm: u32,
    },
    ORI {
        rd: Register,
        rs1: Register,
        imm: u32,
    },
    ANDI {
        rd: Register,
        rs1: Register,
        imm: u32,
    },
    SLLI {
        rd: Register,
        rs1: Register,
        shamt: i32,
    },
    SRLI {
        rd: Register,
        rs1: Register,
        shamt: i32,
    },
    SRAI {
        rd: Register,
        rs1: Register,
        shamt: i32,
    },

    ADD {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },
    SUB {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },
    SLL {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },
    SLT {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },
    SLTU {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },
    XOR {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },
    SRL {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },
    SRA {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },
    OR {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },
    AND {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },

    ADDW {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },
    SUBW {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },
    SLLW {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },
    SLTW {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },
    SRLW {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },
    SRAW {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },

    ADDIW {
        rd: Register,
        rs1: Register,
        imm: u32,
    },
    SLTIW {
        rd: Register,
        rs1: Register,
        imm: u32,
    },
    SLLIW {
        rd: Register,
        rs1: Register,
        shamt: i32,
    },
    SRLIW {
        rd: Register,
        rs1: Register,
        shamt: i32,
    },
    SRAIW {
        rd: Register,
        rs1: Register,
        shamt: i32,
    },

    FENCE {
        rd: Register,
        rs1: Register,
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
