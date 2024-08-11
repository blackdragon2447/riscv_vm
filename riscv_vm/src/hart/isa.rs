use enumflags2::{bitflags, BitFlags};

use crate::decode::Instruction;

#[repr(u64)]
#[bitflags]
#[derive(Clone, Copy, Debug)]
pub enum Isa {
    A = 0x1,
    B = 0x2,
    C = 0x4,
    D = 0x8,
    E = 0x10,
    F = 0x20,
    H = 0x40,
    I = 0x80,
    J = 0x100,
    K = 0x200,
    L = 0x400,
    M = 0x800,
    N = 0x1000,
    O = 0x2000,
    P = 0x4000,
    Q = 0x8000,
    R = 0x10000,
    S = 0x20000,
    T = 0x40000,
    U = 0x80000,
    V = 0x100000,
    W = 0x200000,
    X = 0x400000,
    Y = 0x800000,
    Z = 0x1000000,
}

impl Isa {
    pub fn maximal() -> BitFlags<Self> {
        if cfg!(feature = "float") {
            Self::I | Self::M | Self::A | Self::F | Self::D | Self::C | Self::S | Self::U
        } else {
            Self::I | Self::M | Self::A | Self::C | Self::S | Self::U
        }
    }

    pub fn _validate(bitflags: &mut BitFlags<Self>) {
        *bitflags &= Self::maximal();
    }
}

impl Instruction {
    pub fn extention(&self) -> Isa {
        use Instruction::*;
        match self {
            LUI { .. }
            | AUIPC { .. }
            | JAL { .. }
            | JALR { .. }
            | BEQ { .. }
            | BNE { .. }
            | BLT { .. }
            | BGE { .. }
            | BLTU { .. }
            | BGEU { .. }
            | LB { .. }
            | LH { .. }
            | LW { .. }
            | LD { .. }
            | LBU { .. }
            | LHU { .. }
            | LWU { .. }
            | SB { .. }
            | SH { .. }
            | SW { .. }
            | SD { .. }
            | ADDI { .. }
            | SLTI { .. }
            | SLTIU { .. }
            | XORI { .. }
            | ORI { .. }
            | ANDI { .. }
            | SLLI { .. }
            | SRLI { .. }
            | SRAI { .. }
            | ADD { .. }
            | SUB { .. }
            | SLL { .. }
            | SLT { .. }
            | SLTU { .. }
            | XOR { .. }
            | SRL { .. }
            | SRA { .. }
            | OR { .. }
            | AND { .. }
            | ADDIW { .. }
            | SLTIW { .. }
            | SLLIW { .. }
            | SRLIW { .. }
            | SRAIW { .. }
            | ADDW { .. }
            | SUBW { .. }
            | SLLW { .. }
            | SLTW { .. }
            | SRLW { .. }
            | SRAW { .. }
            | FENCE { .. }
            | ECALL
            | EBREAK => Isa::I,
            MUL { .. }
            | MULH { .. }
            | MULHSU { .. }
            | MULHU { .. }
            | DIV { .. }
            | DIVU { .. }
            | REM { .. }
            | REMU { .. }
            | MULW { .. }
            | DIVW { .. }
            | DIVUW { .. }
            | REMW { .. }
            | REMUW { .. } => Isa::M,
            CSRRW { .. }
            | CSRRS { .. }
            | CSRRC { .. }
            | CSRRWI { .. }
            | CSRRSI { .. }
            | CSRRCI { .. } => Isa::S,
            LR_W { .. }
            | SC_W { .. }
            | AMOSWAP_W { .. }
            | AMOADD_W { .. }
            | AMOXOR_W { .. }
            | AMOAND_W { .. }
            | AMOOR_W { .. }
            | AMOMIN_W { .. }
            | AMOMAX_W { .. }
            | AMOMINU_W { .. }
            | AMOMAXU_W { .. }
            | LR_D { .. }
            | SC_D { .. }
            | AMOSWAP_D { .. }
            | AMOADD_D { .. }
            | AMOXOR_D { .. }
            | AMOAND_D { .. }
            | AMOOR_D { .. }
            | AMOMIN_D { .. }
            | AMOMAX_D { .. }
            | AMOMINU_D { .. }
            | AMOMAXU_D { .. } => Isa::A,
            FLW { .. }
            | FSW { .. }
            | FMADD_S { .. }
            | FMSUB_S { .. }
            | FNMADD_S { .. }
            | FNMSUB_S { .. }
            | FADD_S { .. }
            | FSUB_S { .. }
            | FMUL_S { .. }
            | FDIV_S { .. }
            | FSQRT_S { .. }
            | FSGNJ_S { .. }
            | FSGNJN_S { .. }
            | FSGNJX_S { .. }
            | FMIN_S { .. }
            | FMAX_S { .. }
            | FCVT_W_S { .. }
            | FCVT_WU_S { .. }
            | FMV_X_W { .. }
            | FEQ_S { .. }
            | FLT_S { .. }
            | FLE_S { .. }
            | FCLASS_S { .. }
            | FCVT_S_W { .. }
            | FCVT_S_WU { .. }
            | FMV_W_X { .. }
            | FCVT_L_S { .. }
            | FCVT_LU_S { .. }
            | FCVT_S_L { .. }
            | FCVT_S_LU { .. } => Isa::F,
            FLD { .. }
            | FSD { .. }
            | FMADD_D { .. }
            | FMSUB_D { .. }
            | FNMADD_D { .. }
            | FNMSUB_D { .. }
            | FADD_D { .. }
            | FSUB_D { .. }
            | FMUL_D { .. }
            | FDIV_D { .. }
            | FSQRT_D { .. }
            | FSGNJ_D { .. }
            | FSGNJN_D { .. }
            | FSGNJX_D { .. }
            | FMIN_D { .. }
            | FMAX_D { .. }
            | FCVT_S_D { .. }
            | FCVT_D_S { .. }
            | FEQ_D { .. }
            | FLT_D { .. }
            | FLE_D { .. }
            | FCLASS_D { .. }
            | FCVT_W_D { .. }
            | FCVT_WU_D { .. }
            | FCVT_D_W { .. }
            | FCVT_D_WU { .. }
            | FCVT_L_D { .. }
            | FCVT_LU_D { .. }
            | FMV_X_D { .. }
            | FCVT_D_L { .. }
            | FCVT_D_LU { .. }
            | FMV_D_X { .. } => Isa::D,
            MRET | SRET | WFI | Undifined(_) => Isa::S,
        }
    }
}
