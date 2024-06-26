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
}
