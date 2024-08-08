use crate::hart::{registers::IntRegister, CsrAddress};

#[cfg(feature = "float")]
use crate::hart::registers::FloatRegister;

#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg(feature = "float")]
pub enum RoundingMode {
    ToNearestTieEven = 0b000,
    ToZero = 0b001,
    Down = 0b010,
    Up = 0b011,
    ToNearestTieMagnitude = 0b100,
    Dynamic = 0b111,
}

#[cfg(feature = "float")]
impl TryFrom<u32> for RoundingMode {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0b000 => Ok(Self::ToNearestTieEven),
            0b001 => Ok(Self::ToZero),
            0b010 => Ok(Self::Down),
            0b011 => Ok(Self::Up),
            0b100 => Ok(Self::ToNearestTieMagnitude),
            0b111 => Ok(Self::Dynamic),
            _ => Err(()),
        }
    }
}

#[cfg(feature = "float")]
impl From<RoundingMode> for softfloat_wrapper::RoundingMode {
    fn from(value: RoundingMode) -> Self {
        match value {
            RoundingMode::ToNearestTieEven => Self::TiesToEven,
            RoundingMode::ToZero => Self::TowardZero,
            RoundingMode::Down => Self::TowardNegative,
            RoundingMode::Up => Self::TowardPositive,
            RoundingMode::ToNearestTieMagnitude => Self::TiesToAway,
            RoundingMode::Dynamic => unreachable!(),
        }
    }
}

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

    // RV32F
    #[cfg(feature = "float")]
    FLW {
        rd: FloatRegister,
        rs1: IntRegister,
        imm: i32,
    },
    #[cfg(feature = "float")]
    FSW {
        rs1: IntRegister,
        rs2: FloatRegister,
        imm: i32,
    },
    #[cfg(feature = "float")]
    FMADD_S {
        rd: FloatRegister,
        rs1: FloatRegister,
        rs2: FloatRegister,
        rs3: FloatRegister,
        rm: RoundingMode,
    },
    #[cfg(feature = "float")]
    FMSUB_S {
        rd: FloatRegister,
        rs1: FloatRegister,
        rs2: FloatRegister,
        rs3: FloatRegister,
        rm: RoundingMode,
    },
    #[cfg(feature = "float")]
    FNMADD_S {
        rd: FloatRegister,
        rs1: FloatRegister,
        rs2: FloatRegister,
        rs3: FloatRegister,
        rm: RoundingMode,
    },
    #[cfg(feature = "float")]
    FNMSUB_S {
        rd: FloatRegister,
        rs1: FloatRegister,
        rs2: FloatRegister,
        rs3: FloatRegister,
        rm: RoundingMode,
    },
    #[cfg(feature = "float")]
    FADD_S {
        rd: FloatRegister,
        rs1: FloatRegister,
        rs2: FloatRegister,
        rm: RoundingMode,
    },
    #[cfg(feature = "float")]
    FSUB_S {
        rd: FloatRegister,
        rs1: FloatRegister,
        rs2: FloatRegister,
        rm: RoundingMode,
    },
    #[cfg(feature = "float")]
    FMUL_S {
        rd: FloatRegister,
        rs1: FloatRegister,
        rs2: FloatRegister,
        rm: RoundingMode,
    },
    #[cfg(feature = "float")]
    FDIV_S {
        rd: FloatRegister,
        rs1: FloatRegister,
        rs2: FloatRegister,
        rm: RoundingMode,
    },
    #[cfg(feature = "float")]
    FSQRT_S {
        rd: FloatRegister,
        rs1: FloatRegister,
        rm: RoundingMode,
    },
    #[cfg(feature = "float")]
    FSGNJ_S {
        rd: FloatRegister,
        rs1: FloatRegister,
        rs2: FloatRegister,
    },
    #[cfg(feature = "float")]
    FSGNJN_S {
        rd: FloatRegister,
        rs1: FloatRegister,
        rs2: FloatRegister,
    },
    #[cfg(feature = "float")]
    FSGNJX_S {
        rd: FloatRegister,
        rs1: FloatRegister,
        rs2: FloatRegister,
    },
    #[cfg(feature = "float")]
    FMIN_S {
        rd: FloatRegister,
        rs1: FloatRegister,
        rs2: FloatRegister,
    },
    #[cfg(feature = "float")]
    FMAX_S {
        rd: FloatRegister,
        rs1: FloatRegister,
        rs2: FloatRegister,
    },
    #[cfg(feature = "float")]
    FCVT_W_S {
        rd: IntRegister,
        rs1: FloatRegister,
        rm: RoundingMode,
    },
    #[cfg(feature = "float")]
    FCVT_WU_S {
        rd: IntRegister,
        rs1: FloatRegister,
        rm: RoundingMode,
    },
    #[cfg(feature = "float")]
    FMV_X_W {
        rd: IntRegister,
        rs1: FloatRegister,
    },
    #[cfg(feature = "float")]
    FEQ_S {
        rd: IntRegister,
        rs1: FloatRegister,
        rs2: FloatRegister,
    },
    #[cfg(feature = "float")]
    FLT_S {
        rd: IntRegister,
        rs1: FloatRegister,
        rs2: FloatRegister,
    },
    #[cfg(feature = "float")]
    FLE_S {
        rd: IntRegister,
        rs1: FloatRegister,
        rs2: FloatRegister,
    },
    #[cfg(feature = "float")]
    FCLASS_S {
        rd: IntRegister,
        rs1: FloatRegister,
    },
    #[cfg(feature = "float")]
    FCVT_S_W {
        rd: FloatRegister,
        rs1: IntRegister,
        rm: RoundingMode,
    },
    #[cfg(feature = "float")]
    FCVT_S_WU {
        rd: FloatRegister,
        rs1: IntRegister,
        rm: RoundingMode,
    },
    #[cfg(feature = "float")]
    FMV_W_X {
        rd: FloatRegister,
        rs1: IntRegister,
    },

    // RV64F
    #[cfg(feature = "float")]
    FCVT_L_S {
        rd: IntRegister,
        rs1: FloatRegister,
        rm: RoundingMode,
    },
    #[cfg(feature = "float")]
    FCVT_LU_S {
        rd: IntRegister,
        rs1: FloatRegister,
        rm: RoundingMode,
    },
    #[cfg(feature = "float")]
    FCVT_S_L {
        rd: FloatRegister,
        rs1: IntRegister,
        rm: RoundingMode,
    },
    #[cfg(feature = "float")]
    FCVT_S_LU {
        rd: FloatRegister,
        rs1: IntRegister,
        rm: RoundingMode,
    },

    // Privilege
    MRET,
    SRET,
    WFI,

    Undifined(u32),
}
