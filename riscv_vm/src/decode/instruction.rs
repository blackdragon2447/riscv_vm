use crate::{hart::registers::IntRegister, hart::CsrAddress};

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Instruction {
    // RV64I
    LUI {
        rd: IntRegister,
        imm: i32,
    },
    AUIPC {
        rd: IntRegister,
        imm: i32,
    },

    JAL {
        rd: IntRegister,
        imm: i32,
    },
    JALR {
        rd: IntRegister,
        rs1: IntRegister,
        imm: i32,
    },

    BEQ {
        rs1: IntRegister,
        rs2: IntRegister,
        imm: i32,
    },
    BNE {
        rs1: IntRegister,
        rs2: IntRegister,
        imm: i32,
    },
    BLT {
        rs1: IntRegister,
        rs2: IntRegister,
        imm: i32,
    },
    BGE {
        rs1: IntRegister,
        rs2: IntRegister,
        imm: i32,
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
        imm: i32,
    },
    LH {
        rd: IntRegister,
        rs1: IntRegister,
        imm: i32,
    },
    LW {
        rd: IntRegister,
        rs1: IntRegister,
        imm: i32,
    },
    LD {
        rd: IntRegister,
        rs1: IntRegister,
        imm: i32,
    },
    LBU {
        rd: IntRegister,
        rs1: IntRegister,
        imm: i32,
    },
    LHU {
        rd: IntRegister,
        rs1: IntRegister,
        imm: i32,
    },
    LWU {
        rd: IntRegister,
        rs1: IntRegister,
        imm: i32,
    },
    SB {
        rs1: IntRegister,
        rs2: IntRegister,
        imm: i32,
    },
    SH {
        rs1: IntRegister,
        rs2: IntRegister,
        imm: i32,
    },
    SW {
        rs1: IntRegister,
        rs2: IntRegister,
        imm: i32,
    },
    SD {
        rs1: IntRegister,
        rs2: IntRegister,
        imm: i32,
    },

    ADDI {
        rd: IntRegister,
        rs1: IntRegister,
        imm: i32,
    },
    SLTI {
        rd: IntRegister,
        rs1: IntRegister,
        imm: i32,
    },
    SLTIU {
        rd: IntRegister,
        rs1: IntRegister,
        imm: u32,
    },
    XORI {
        rd: IntRegister,
        rs1: IntRegister,
        imm: i32,
    },
    ORI {
        rd: IntRegister,
        rs1: IntRegister,
        imm: i32,
    },
    ANDI {
        rd: IntRegister,
        rs1: IntRegister,
        imm: i32,
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

    ADDIW {
        rd: IntRegister,
        rs1: IntRegister,
        imm: i32,
    },
    SLTIW {
        rd: IntRegister,
        rs1: IntRegister,
        imm: i32,
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

    FENCE {
        rd: IntRegister,
        rs1: IntRegister,
        imm: i32,
    },
    ECALL,
    EBREAK,

    //RV64M
    MUL {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
    },
    MULH {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
    },
    MULHSU {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
    },
    MULHU {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
    },
    DIV {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
    },
    DIVU {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
    },
    REM {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
    },
    REMU {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
    },

    MULW {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
    },
    DIVW {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
    },
    DIVUW {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
    },
    REMW {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
    },
    REMUW {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
    },

    // RV64 Zicsr
    CSRRW {
        rd: IntRegister,
        rs1: IntRegister,
        csr: CsrAddress,
    },
    CSRRS {
        rd: IntRegister,
        rs1: IntRegister,
        csr: CsrAddress,
    },
    CSRRC {
        rd: IntRegister,
        rs1: IntRegister,
        csr: CsrAddress,
    },
    CSRRWI {
        rd: IntRegister,
        uimm: u32,
        csr: CsrAddress,
    },
    CSRRSI {
        rd: IntRegister,
        uimm: u32,
        csr: CsrAddress,
    },
    CSRRCI {
        rd: IntRegister,
        uimm: u32,
        csr: CsrAddress,
    },

    // RV32A
    LR_W {
        rd: IntRegister,
        rs1: IntRegister,
        rl: bool,
        aq: bool,
    },
    SC_W {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
        rl: bool,
        aq: bool,
    },
    AMOSWAP_W {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
        rl: bool,
        aq: bool,
    },
    AMOADD_W {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
        rl: bool,
        aq: bool,
    },
    AMOXOR_W {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
        rl: bool,
        aq: bool,
    },
    AMOAND_W {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
        rl: bool,
        aq: bool,
    },
    AMOOR_W {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
        rl: bool,
        aq: bool,
    },
    AMOMIN_W {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
        rl: bool,
        aq: bool,
    },
    AMOMAX_W {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
        rl: bool,
        aq: bool,
    },
    AMOMINU_W {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
        rl: bool,
        aq: bool,
    },
    AMOMAXU_W {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
        rl: bool,
        aq: bool,
    },

    // RV64A
    LR_D {
        rd: IntRegister,
        rs1: IntRegister,
        rl: bool,
        aq: bool,
    },
    SC_D {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
        rl: bool,
        aq: bool,
    },
    AMOSWAP_D {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
        rl: bool,
        aq: bool,
    },
    AMOADD_D {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
        rl: bool,
        aq: bool,
    },
    AMOXOR_D {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
        rl: bool,
        aq: bool,
    },
    AMOAND_D {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
        rl: bool,
        aq: bool,
    },
    AMOOR_D {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
        rl: bool,
        aq: bool,
    },
    AMOMIN_D {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
        rl: bool,
        aq: bool,
    },
    AMOMAX_D {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
        rl: bool,
        aq: bool,
    },
    AMOMINU_D {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
        rl: bool,
        aq: bool,
    },
    AMOMAXU_D {
        rd: IntRegister,
        rs1: IntRegister,
        rs2: IntRegister,
        rl: bool,
        aq: bool,
    },

    // Privilege
    MRET,
    SRET,
    WFI,

    Undifined(u32),
}
