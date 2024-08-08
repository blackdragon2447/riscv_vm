mod types {
    use crate::decode::{get_type_and_opcode, InstructionType};

    #[test]
    fn lui_type() {
        assert_eq!(
            get_type_and_opcode(0x0dead037),
            Some((InstructionType::U, 0b0110111))
        );
    }

    #[test]
    fn jal_type() {
        assert_eq!(
            get_type_and_opcode(0x000000ef),
            Some((InstructionType::J, 0b1101111))
        )
    }

    #[test]
    fn blt_type() {
        assert_eq!(
            get_type_and_opcode(0x000c4463),
            Some((InstructionType::B, 0b1100011))
        )
    }

    #[test]
    fn add_type() {
        assert_eq!(
            get_type_and_opcode(0x00418133),
            Some((InstructionType::R, 0b0110011))
        )
    }
}

mod instructions {
    use crate::{
        decode::{decode, Instruction::*},
        hart::registers::IntRegister::*,
        tests,
    };

    #[cfg(feature = "float")]
    use crate::{decode::instruction::RoundingMode, hart::registers::FloatRegister::*};

    #[test]
    fn aiupc() {
        assert_eq!(decode(0x00000117), AUIPC { rd: X2, imm: 0x0 });
    }

    #[test]
    fn mv_add() {
        assert_eq!(
            decode(0x00010113),
            ADDI {
                rd: X2,
                rs1: X2,
                imm: 0x0
            }
        )
    }

    #[test]
    fn lui() {
        assert_eq!(
            decode(0x00001537),
            LUI {
                rd: X10,
                imm: 0x1 << 12
            }
        )
    }

    #[test]
    fn addi() {
        assert_eq!(
            decode(0x00158593),
            ADDI {
                rd: X11,
                rs1: X11,
                imm: 0x1
            }
        )
    }

    #[test]
    fn add() {
        assert_eq!(
            decode(0x00a10133),
            ADD {
                rd: X2,
                rs1: X2,
                rs2: X10
            }
        )
    }

    #[test]
    fn jalr() {
        assert_eq!(
            decode(0x000080e7),
            JALR {
                rd: X1,
                rs1: X1,
                imm: 0x0
            }
        )
    }

    #[test]
    fn jal() {
        assert_eq!(
            decode(0x0b80206f),
            JAL {
                rd: X0,
                imm: 0x20b8
            }
        );
    }

    #[test]
    fn addiw() {
        assert_eq!(
            decode(0xead5859b),
            ADDIW {
                rd: X11,
                rs1: X11,
                imm: -339
            }
        )
    }

    #[test]
    fn mul() {
        assert_eq!(
            decode(0x02c58533),
            MUL {
                rd: X10,
                rs1: X11,
                rs2: X12
            }
        )
    }

    #[test]
    fn mret() {
        assert_eq!(decode(0x30200073), MRET)
    }

    #[test]
    fn srai() {
        assert_eq!(
            decode(0x4010d093),
            SRAI {
                rd: X1,
                rs1: X1,
                shamt: 1,
            }
        )
    }

    #[test]
    fn sd() {
        assert_eq!(
            decode(0xfef83c23),
            SD {
                rs1: X16,
                rs2: X15,
                imm: -8,
            }
        )
    }

    #[test]
    fn amoadd_d() {
        assert_eq!(
            decode(0x00b6b72f),
            AMOADD_D {
                rd: X14,
                rs1: X13,
                rs2: X11,
                rl: false,
                aq: false
            }
        )
    }

    #[test]
    fn lr_w() {
        assert_eq!(
            decode(0x1005272f),
            LR_W {
                rd: X14,
                rs1: X10,
                rl: false,
                aq: false
            }
        )
    }

    #[test]
    fn srl() {
        assert_eq!(
            decode(0x0207d793),
            SRLI {
                rd: X15,
                rs1: X15,
                shamt: 0x20
            }
        )
    }

    #[test]
    #[cfg(feature = "float")]
    fn fsqrt_s() {
        assert_eq!(
            decode(0x580130d3),
            FSQRT_S {
                rd: F1,
                rs1: F2,
                rm: RoundingMode::Up,
            }
        );
    }

    #[test]
    #[cfg(feature = "float")]
    fn fclass_s() {
        assert_eq!(decode(0xe0009153), FCLASS_S { rd: X2, rs1: F1 });
    }

    #[test]
    #[cfg(feature = "float")]
    fn flw() {
        assert_eq!(
            decode(0x0250a187),
            FLW {
                rd: F3,
                rs1: X1,
                imm: 37
            }
        );
    }

    #[test]
    #[cfg(feature = "float")]
    fn fmadd_s() {
        assert_eq!(
            decode(0x484381c3),
            FMADD_S {
                rd: F3,
                rs1: F7,
                rs2: F4,
                rs3: F9,
                rm: RoundingMode::ToNearestTieEven
            }
        )
    }

    #[test]
    #[cfg(feature = "float")]
    fn fmul_s() {
        assert_eq!(
            decode(0x10c47253),
            FMUL_S {
                rd: F4,
                rs1: F8,
                rs2: F12,
                rm: RoundingMode::Dynamic
            }
        );
    }

    #[test]
    #[cfg(feature = "float")]
    fn feq_s() {
        assert_eq!(
            decode(0xa149a7d3),
            FEQ_S {
                rd: X15,
                rs1: F19,
                rs2: F20
            }
        )
    }

    #[test]
    #[cfg(feature = "float")]
    fn fdiv_s() {
        assert_eq!(
            decode(0x181071d3),
            FDIV_S {
                rd: F3,
                rs1: F0,
                rs2: F1,
                rm: RoundingMode::Dynamic
            }
        )
    }

    #[test]
    fn fcvt_lu_s() {
        assert_eq!(
            decode(0xd0357053),
            FCVT_S_LU {
                rd: F0,
                rs1: X10,
                rm: RoundingMode::Dynamic,
            }
        );
    }

    #[test]
    fn fsw() {
        assert_eq!(
            decode(0x0015aa27),
            FSW {
                rs1: X11,
                rs2: F1,
                imm: 20,
            }
        );
    }
}
