use crate::hart::registers::IntRegister;

use super::Instruction;

pub fn decode_compact(inst: u16) -> Instruction {
    println!("{:#018b}", inst);
    let opcode = inst & 0b11;
    let funct3 = (inst >> 13) & 0b111;

    match (opcode, funct3) {
        (0b00, 0b000) => {
            let imm_3 = (inst >> 5) & 0b1;
            let imm_2 = (inst >> 6) & 0b1;
            let imm_6_9 = (inst >> 7) & 0b1111;
            let imm_4_5 = (inst >> 11) & 0b11;
            let imm = ((imm_2 << 2) | (imm_3 << 3) | (imm_4_5) << 4 | (imm_6_9 << 6)) as u32 as i32;
            let rd = (inst >> 2) & 0b111;
            Instruction::ADDI {
                rd: rd.into(),
                rs1: IntRegister::X2,
                imm,
            }
        }
        #[cfg(feature = "float")]
        (0b00, 0b001) => {
            let rd = (inst >> 2) & 0b111;
            let rs1 = (inst >> 7) & 0b111;
            Instruction::FLD {
                rd: rd.into(),
                rs1: rs1.into(),
                imm: cl_double_imm(inst),
            }
        }
        (0b00, 0b010) => {
            let rd = (inst >> 2) & 0b111;
            let rs1 = (inst >> 7) & 0b111;
            Instruction::LW {
                rd: rd.into(),
                rs1: rs1.into(),
                imm: cl_word_imm(inst),
            }
        }
        (0b00, 0b011) => {
            let rd = (inst >> 2) & 0b111;
            let rs1 = (inst >> 7) & 0b111;
            Instruction::LD {
                rd: rd.into(),
                rs1: rs1.into(),
                imm: cl_double_imm(inst),
            }
        }
        #[cfg(feature = "float")]
        (0b00, 0b101) => {
            let rs2 = (inst >> 2) & 0b111;
            let rs1 = (inst >> 7) & 0b111;
            Instruction::FSD {
                rs2: rs2.into(),
                rs1: rs1.into(),
                imm: cl_double_imm(inst),
            }
        }
        (0b00, 0b110) => {
            let rs2 = (inst >> 2) & 0b111;
            let rs1 = (inst >> 7) & 0b111;
            Instruction::SW {
                rs2: rs2.into(),
                rs1: rs1.into(),
                imm: cl_word_imm(inst),
            }
        }
        (0b00, 0b111) => {
            let rs2 = (inst >> 2) & 0b111;
            let rs1 = (inst >> 7) & 0b111;
            Instruction::SD {
                rs2: rs2.into(),
                rs1: rs1.into(),
                imm: cl_double_imm(inst),
            }
        }
        (0b01, 0b000) => {
            let rd_rs1 = ((inst >> 7) & 0b11111) as u32;
            // if imm == 0 {
            // Instruction::Undifined(inst as u32)
            // } else {
            Instruction::ADDI {
                rd: rd_rs1.into(),
                rs1: rd_rs1.into(),
                imm: ci_imm(inst),
            }
            // }
        }
        (0b01, 0b001) => {
            let rd_rs1 = ((inst >> 7) & 0b11111) as u32;
            if rd_rs1 == 0 {
                Instruction::Undifined(inst as u32)
            } else {
                Instruction::ADDIW {
                    rd: rd_rs1.into(),
                    rs1: rd_rs1.into(),
                    imm: ci_imm(inst),
                }
            }
        }
        (0b01, 0b010) => {
            let rd = ((inst >> 7) & 0b11111) as u32;
            if rd == 0 {
                Instruction::Undifined(inst as u32)
            } else {
                Instruction::ADDIW {
                    rd: rd.into(),
                    rs1: IntRegister::X0,
                    imm: ci_imm(inst),
                }
            }
        }
        (0b01, 0b011) => {
            let rd = ((inst >> 7) & 0b11111) as u32;
            let imm = ci_imm(inst) << 12;
            if rd == 0 || imm == 0 {
                Instruction::Undifined(inst as u32)
            } else if rd == 2 {
                let imm_5 = (inst >> 2) & 0b1;
                let imm_7_8 = (inst >> 3) & 0b11;
                let imm_6 = (inst >> 5) & 0b1;
                let imm_4 = (inst >> 6) & 0b1;
                let imm_9 = (inst >> 12) & 0b1;
                let imm =
                    ((((imm_4 << 4) | (imm_5 << 5) | (imm_6 << 6) | (imm_7_8 << 7) | (imm_9 << 9))
                        as i32)
                        << 22)
                        >> 22;
                Instruction::ADDI {
                    rd: rd.into(),
                    rs1: IntRegister::X2,
                    imm,
                }
            } else {
                Instruction::LUI { rd: rd.into(), imm }
            }
        }
        (0b01, 0b100) => {
            let funct2 = (inst >> 10) & 0b11;
            match funct2 {
                0b00 => {
                    let rd_rs1 = (inst >> 7) & 0b111;
                    let shamt = ci_shamt(inst);
                    if shamt == 0 {
                        Instruction::Undifined(inst as u32)
                    } else {
                        Instruction::SRLI {
                            rd: rd_rs1.into(),
                            rs1: rd_rs1.into(),
                            shamt,
                        }
                    }
                }
                0b01 => {
                    let rd_rs1 = (inst >> 7) & 0b111;
                    let shamt = ci_shamt(inst);
                    if shamt == 0 {
                        Instruction::Undifined(inst as u32)
                    } else {
                        Instruction::SRAI {
                            rd: rd_rs1.into(),
                            rs1: rd_rs1.into(),
                            shamt,
                        }
                    }
                }
                0b10 => {
                    let rd_rs1 = (inst >> 7) & 0b111;
                    Instruction::ANDI {
                        rd: rd_rs1.into(),
                        rs1: rd_rs1.into(),
                        imm: ci_imm(inst),
                    }
                }
                0b11 => {
                    let funct1 = (inst >> 12) & 0b1;
                    let funct2 = (inst >> 5) & 0b11;
                    let rd_rs1 = (inst >> 7) & 0b111;
                    let rs2 = (inst >> 2) & 0b111;
                    match (funct1, funct2) {
                        (0b0, 0b00) => Instruction::SUB {
                            rd: rd_rs1.into(),
                            rs1: rd_rs1.into(),
                            rs2: rs2.into(),
                        },
                        (0b0, 0b01) => Instruction::XOR {
                            rd: rd_rs1.into(),
                            rs1: rd_rs1.into(),
                            rs2: rs2.into(),
                        },
                        (0b0, 0b10) => Instruction::OR {
                            rd: rd_rs1.into(),
                            rs1: rd_rs1.into(),
                            rs2: rs2.into(),
                        },
                        (0b0, 0b11) => Instruction::AND {
                            rd: rd_rs1.into(),
                            rs1: rd_rs1.into(),
                            rs2: rs2.into(),
                        },
                        (0b1, 0b00) => Instruction::SUBW {
                            rd: rd_rs1.into(),
                            rs1: rd_rs1.into(),
                            rs2: rs2.into(),
                        },
                        (0b1, 0b01) => Instruction::ADDW {
                            rd: rd_rs1.into(),
                            rs1: rd_rs1.into(),
                            rs2: rs2.into(),
                        },
                        _ => Instruction::Undifined(inst as u32),
                    }
                }
                _ => unreachable!(),
            }
        }
        (0b01, 0b101) => {
            let imm_5 = (inst >> 2) & 0b1;
            let imm_1_3 = (inst >> 3) & 0b111;
            let imm_7 = (inst >> 6) & 0b1;
            let imm_6 = (inst >> 7) & 0b1;
            let imm_10 = (inst >> 8) & 0b1;
            let imm_8_9 = (inst >> 9) & 0b11;
            let imm_4 = (inst >> 11) & 0b1;
            let imm_11 = (inst >> 12) & 0b1;
            let imm = ((((imm_1_3 << 1)
                | (imm_4 << 4)
                | (imm_5 << 5)
                | (imm_6 << 6)
                | (imm_7 << 7)
                | (imm_8_9 << 8)
                | (imm_10 << 10)
                | (imm_11 << 11)) as i32)
                << 20)
                >> 20;
            Instruction::JAL {
                rd: IntRegister::X0,
                imm,
            }
        }
        (0b01, 0b110) => {
            let rs1 = (inst >> 7) & 0b111;
            Instruction::BEQ {
                rs1: rs1.into(),
                rs2: IntRegister::X0,
                imm: cb_imm(inst),
            }
        }
        (0b01, 0b111) => {
            let rs1 = (inst >> 7) & 0b111;
            Instruction::BNE {
                rs1: rs1.into(),
                rs2: IntRegister::X0,
                imm: cb_imm(inst),
            }
        }
        (0b10, 0b000) => {
            let rd_rs1 = (inst >> 7) & 0b111;
            let shamt = ci_shamt(inst);
            if shamt == 0 {
                Instruction::Undifined(inst as u32)
            } else {
                Instruction::SLLI {
                    rd: rd_rs1.into(),
                    rs1: rd_rs1.into(),
                    shamt,
                }
            }
        }
        #[cfg(feature = "float")]
        (0b10, 0b001) => {
            let rd = ((inst >> 7) & 0b11111) as u32;
            Instruction::FLD {
                rd: rd.into(),
                rs1: IntRegister::X2,
                imm: ci_sp_double_imm(inst),
            }
        }
        (0b10, 0b010) => {
            let rd = ((inst >> 7) & 0b11111) as u32;
            Instruction::LW {
                rd: rd.into(),
                rs1: IntRegister::X2,
                imm: ci_sp_word_imm(inst),
            }
        }
        (0b10, 0b011) => {
            let rd = ((inst >> 7) & 0b11111) as u32;
            Instruction::LD {
                rd: rd.into(),
                rs1: IntRegister::X2,
                imm: ci_sp_double_imm(inst),
            }
        }
        (0b10, 0b100) => {
            let funct1 = (inst >> 12) & 0b1;
            let rd_rs1 = ((inst >> 7) & 0b11111) as u32;
            let rs2 = ((inst >> 2) & 0b11111) as u32;
            if funct1 == 0 && rd_rs1 != 0 && rs2 == 0 {
                Instruction::JALR {
                    rd: IntRegister::X0,
                    rs1: rd_rs1.into(),
                    imm: 0,
                }
            } else if funct1 == 0 && rd_rs1 != 0 && rs2 != 0 {
                Instruction::ADD {
                    rd: rd_rs1.into(),
                    rs1: IntRegister::X0,
                    rs2: rs2.into(),
                }
            } else if funct1 == 1 && rd_rs1 != 0 && rs2 == 0 {
                Instruction::JALR {
                    rd: IntRegister::X1,
                    rs1: rd_rs1.into(),
                    imm: 0,
                }
            } else if funct1 == 1 && rd_rs1 != 0 && rs2 != 0 {
                Instruction::ADD {
                    rd: rd_rs1.into(),
                    rs1: rd_rs1.into(),
                    rs2: rs2.into(),
                }
            } else if funct1 == 1 && rd_rs1 == 0 && rs2 == 0 {
                Instruction::EBREAK
            } else {
                Instruction::Undifined(inst as u32)
            }
        }
        #[cfg(feature = "float")]
        (0b10, 0b101) => {
            let rs2 = ((inst >> 7) & 0b11111) as u32;
            Instruction::FSD {
                rs1: IntRegister::X2,
                rs2: rs2.into(),
                imm: ci_sp_double_imm(inst),
            }
        }
        (0b10, 0b110) => {
            let rs2 = ((inst >> 2) & 0b11111) as u32;
            let imm_6_7 = (inst >> 7) & 0b11;
            let imm_2_5 = (inst >> 9) & 0b1111;
            let imm = ((imm_2_5 << 2) | (imm_6_7 << 6)) as i32;
            Instruction::SW {
                rs1: IntRegister::X2,
                rs2: rs2.into(),
                imm,
            }
        }
        (0b10, 0b111) => {
            let rs2 = ((inst >> 2) & 0b11111) as u32;
            let imm_6_8 = (inst >> 7) & 0b111;
            let imm_3_5 = (inst >> 10) & 0b111;
            let imm = ((imm_3_5 << 3) | (imm_6_8 << 6)) as i32;
            Instruction::SD {
                rs1: IntRegister::X2,
                rs2: rs2.into(),
                imm,
            }
        }
        _ => Instruction::Undifined(inst as u32),
    }
}

fn cl_double_imm(inst: u16) -> i32 {
    let imm_3_5 = (inst >> 10) & 0b111;
    let imm_6_7 = (inst >> 5) & 0b11;
    let imm = ((imm_3_5 << 3) | (imm_6_7 << 6)) as u32 as i32;
    imm
}

fn cl_word_imm(inst: u16) -> i32 {
    let imm_3_5 = (inst >> 10) & 0b111;
    let imm_6 = (inst >> 5) & 0b1;
    let imm_2 = (inst >> 6) & 0b1;
    let imm = ((imm_2 << 2) | (imm_3_5 << 3) | (imm_6 << 6)) as u32 as i32;
    imm
}

fn ci_imm(inst: u16) -> i32 {
    let imm_0_4 = (inst >> 2) & 0b11111;
    let imm_5 = (inst >> 12) & 0b1;
    let imm = (((imm_0_4 | (imm_5 << 5)) as i32) << 26) >> 26;
    imm
}

fn ci_shamt(inst: u16) -> i32 {
    let shamt_0_4 = (inst >> 2) & 0b1111;
    let shamt_5 = (inst >> 12) & 0b1;
    let shamt = (shamt_0_4 | (shamt_5 << 5)) as i32;
    shamt
}

fn ci_sp_double_imm(inst: u16) -> i32 {
    let imm_6_8 = (inst >> 2) & 0b111;
    let imm_3_4 = (inst >> 5) & 0b11;
    let imm_5 = (inst >> 12) & 0b1;
    let imm = ((imm_3_4 << 3) | (imm_5 << 5) | (imm_6_8 << 6)) as i32;
    imm
}

fn ci_sp_word_imm(inst: u16) -> i32 {
    let imm_6_7 = (inst >> 2) & 0b11;
    let imm_2_4 = (inst >> 4) & 0b111;
    let imm_5 = (inst >> 12) & 0b1;
    let imm = ((imm_2_4 << 2) | (imm_5 << 5) | (imm_6_7 << 6)) as i32;
    imm
}

fn cb_imm(inst: u16) -> i32 {
    let imm_5 = (inst >> 2) & 0b1;
    let imm_1_2 = (inst >> 3) & 0b11;
    let imm_6_7 = (inst >> 5) & 0b11;
    let imm_3_4 = (inst >> 10) & 0b11;
    let imm_8 = (inst >> 12) & 0b1;
    let imm = ((((imm_1_2 << 1) | (imm_3_4 << 3) | (imm_5 << 5) | (imm_6_7 << 6) | (imm_8 << 8))
        as i32)
        << 23)
        >> 23;
    imm
}
