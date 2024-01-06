use self::instruction::Instruction;

pub mod instruction;
#[cfg(test)]
mod tests;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum InstructionType {
    R,
    I,
    S,
    B,
    U,
    J,
}

pub fn decode(inst: u32) -> Instruction {
    if let Some((inst_type, opcode)) = get_type_and_opcode(inst) {
        match inst_type {
            InstructionType::R => {
                let rd = (inst & masks::RD_MASK) >> 7;
                let funct3 = (inst & masks::FUNCT3_MASK) >> 12;
                let rs1 = (inst & masks::RS1_MASK) >> 15;
                let rs2 = (inst & masks::RS2_MASK) >> 20;
                let funct7 = (inst & masks::FUNCT7_MASK) >> 25;

                match (opcode, funct3, funct7) {
                    (0b0110011, 0b000, 0b0000000) => Instruction::ADD {
                        rd: rd.into(),
                        rs1: rs1.into(),
                        rs2: rs2.into(),
                    },
                    (0b0110011, 0b000, 0b0100000) => Instruction::SUB {
                        rd: rd.into(),
                        rs1: rs1.into(),
                        rs2: rs2.into(),
                    },
                    (0b0110011, 0b001, 0b0000000) => Instruction::SLL {
                        rd: rd.into(),
                        rs1: rs1.into(),
                        rs2: rs2.into(),
                    },
                    (0b0110011, 0b010, 0b0000000) => Instruction::SLT {
                        rd: rd.into(),
                        rs1: rs1.into(),
                        rs2: rs2.into(),
                    },
                    (0b0110011, 0b011, 0b0000000) => Instruction::SLTU {
                        rd: rd.into(),
                        rs1: rs1.into(),
                        rs2: rs2.into(),
                    },
                    (0b0110011, 0b100, 0b0000000) => Instruction::XOR {
                        rd: rd.into(),
                        rs1: rs1.into(),
                        rs2: rs2.into(),
                    },
                    (0b0110011, 0b101, 0b0000000) => Instruction::SRL {
                        rd: rd.into(),
                        rs1: rs1.into(),
                        rs2: rs2.into(),
                    },
                    (0b0110011, 0b101, 0b0100000) => Instruction::SRA {
                        rd: rd.into(),
                        rs1: rs1.into(),
                        rs2: rs2.into(),
                    },
                    (0b0110011, 0b110, 0b0000000) => Instruction::OR {
                        rd: rd.into(),
                        rs1: rs1.into(),
                        rs2: rs2.into(),
                    },
                    (0b0110011, 0b111, 0b0000000) => Instruction::AND {
                        rd: rd.into(),
                        rs1: rs1.into(),
                        rs2: rs2.into(),
                    },
                    (0b0111011, 0b000, 0b0000000) => Instruction::ADDW {
                        rd: rd.into(),
                        rs1: rs1.into(),
                        rs2: rs2.into(),
                    },
                    (0b0111011, 0b000, 0b0100000) => Instruction::SUBW {
                        rd: rd.into(),
                        rs1: rs1.into(),
                        rs2: rs2.into(),
                    },
                    (0b0111011, 0b001, 0b0000000) => Instruction::SLLW {
                        rd: rd.into(),
                        rs1: rs1.into(),
                        rs2: rs2.into(),
                    },
                    (0b0111011, 0b101, 0b0000000) => Instruction::SRLW {
                        rd: rd.into(),
                        rs1: rs1.into(),
                        rs2: rs2.into(),
                    },
                    (0b0111011, 0b101, 0b0100000) => Instruction::SRAW {
                        rd: rd.into(),
                        rs1: rs1.into(),
                        rs2: rs2.into(),
                    },
                    _ => Instruction::Undifined(inst),
                }
            }
            InstructionType::I => {
                let rd = (inst & masks::RD_MASK) >> 7;
                let funct3 = (inst & masks::FUNCT3_MASK) >> 12;
                let rs1 = (inst & masks::RS1_MASK) >> 15;
                let imm11_0 = (inst & masks::IMM11_0_MASK) >> 20;

                let imm = imm11_0 as i32;

                match (opcode, funct3) {
                    (0b1100111, 0b000) => Instruction::JALR {
                        rd: rd.into(),
                        rs1: rs1.into(),
                        imm,
                    },
                    (0b0000011, 0b000) => Instruction::LB {
                        rd: rd.into(),
                        rs1: rs1.into(),
                        imm,
                    },
                    (0b0000011, 0b001) => Instruction::LH {
                        rd: rd.into(),
                        rs1: rs1.into(),
                        imm,
                    },
                    (0b0000011, 0b010) => Instruction::LW {
                        rd: rd.into(),
                        rs1: rs1.into(),
                        imm,
                    },
                    (0b0000011, 0b011) => Instruction::LD {
                        rd: rd.into(),
                        rs1: rs1.into(),
                        imm,
                    },
                    (0b0000011, 0b100) => Instruction::LBU {
                        rd: rd.into(),
                        rs1: rs1.into(),
                        imm,
                    },
                    (0b0000011, 0b101) => Instruction::LHU {
                        rd: rd.into(),
                        rs1: rs1.into(),
                        imm,
                    },
                    (0b0000011, 0b110) => Instruction::LWU {
                        rd: rd.into(),
                        rs1: rs1.into(),
                        imm,
                    },
                    (0b0010011, 0b000) => Instruction::ADDI {
                        rd: rd.into(),
                        rs1: rs1.into(),
                        imm,
                    },
                    (0b0010011, 0b010) => Instruction::SLTI {
                        rd: rd.into(),
                        rs1: rs1.into(),
                        imm,
                    },
                    (0b0010011, 0b011) => Instruction::SLTIU {
                        rd: rd.into(),
                        rs1: rs1.into(),
                        imm: imm as u32,
                    },
                    (0b0010011, 0b100) => Instruction::XORI {
                        rd: rd.into(),
                        rs1: rs1.into(),
                        imm,
                    },
                    (0b0010011, 0b110) => Instruction::ORI {
                        rd: rd.into(),
                        rs1: rs1.into(),
                        imm,
                    },
                    (0b0010011, 0b111) => Instruction::ANDI {
                        rd: rd.into(),
                        rs1: rs1.into(),
                        imm,
                    },
                    (0b0010011, 0b001) => {
                        let shamt = (imm & 0b11_1111);
                        Instruction::SLLI {
                            rd: rd.into(),
                            rs1: rs1.into(),
                            shamt,
                        }
                    }
                    (0b0010011, 0b101) => {
                        let shamt = (imm & 0b11_1111);
                        match imm & 0b1111_1100_0000 {
                            0b0000000 => Instruction::SRLI {
                                rd: rd.into(),
                                rs1: rs1.into(),
                                shamt,
                            },
                            0b0100000 => Instruction::SRAI {
                                rd: rd.into(),
                                rs1: rs1.into(),
                                shamt,
                            },
                            _ => unreachable!(),
                        }
                    }
                    (0b0011011, 0b000) => Instruction::ADDIW {
                        rd: rd.into(),
                        rs1: rs1.into(),
                        imm,
                    },
                    (0b0011011, 0b001) => {
                        let shamt = (imm & 0b11_1111);
                        Instruction::SLLIW {
                            rd: rd.into(),
                            rs1: rs1.into(),
                            shamt,
                        }
                    }
                    (0b0011011, 0b101) => {
                        let shamt = (imm & 0b11_1111);
                        match imm & 0b1111_1100_0000 {
                            0b0000000 => Instruction::SRLIW {
                                rd: rd.into(),
                                rs1: rs1.into(),
                                shamt,
                            },
                            0b0100000 => Instruction::SRAIW {
                                rd: rd.into(),
                                rs1: rs1.into(),
                                shamt,
                            },
                            _ => unreachable!(),
                        }
                    }
                    (0b0001111, 0b000) => Instruction::FENCE {
                        rd: rd.into(),
                        rs1: rs1.into(),
                        imm,
                    },
                    (0b1110011, 0b000) => {
                        if rd == 0 && rs1 == 0 {
                            match imm {
                                0 => Instruction::ECALL,
                                1 => Instruction::EBREAK,
                                _ => Instruction::Undifined(inst),
                            }
                        } else {
                            Instruction::Undifined(inst)
                        }
                    }
                    _ => Instruction::Undifined(inst),
                }
            }
            InstructionType::S => {
                let imm4_0 = (inst & masks::RD_MASK) >> 7;
                let funct3 = (inst & masks::FUNCT3_MASK) >> 12;
                let rs1 = (inst & masks::RS1_MASK) >> 15;
                let rs2 = (inst & masks::RS2_MASK) >> 20;
                let imm11_5 = (inst & masks::FUNCT7_MASK) >> 25;

                let imm = (imm4_0 | (imm11_5 << 5)) as i32;

                match (opcode, funct3) {
                    (0b0100011, 0b000) => Instruction::SB {
                        rs1: rs1.into(),
                        rs2: rs2.into(),
                        imm,
                    },
                    (0b0100011, 0b001) => Instruction::SH {
                        rs1: rs1.into(),
                        rs2: rs2.into(),
                        imm,
                    },
                    (0b0100011, 0b010) => Instruction::SW {
                        rs1: rs1.into(),
                        rs2: rs2.into(),
                        imm,
                    },
                    (0b0100011, 0b011) => Instruction::SD {
                        rs1: rs1.into(),
                        rs2: rs2.into(),
                        imm,
                    },
                    _ => Instruction::Undifined(inst),
                }
            }
            InstructionType::B => {
                let imm4_1_11 = (inst & masks::RD_MASK) >> 7;
                let funct3 = (inst & masks::FUNCT3_MASK) >> 12;
                let rs1 = (inst & masks::RS1_MASK) >> 15;
                let rs2 = (inst & masks::RS2_MASK) >> 20;
                let imm12_10_5 = (inst & masks::FUNCT7_MASK) >> 25;

                let imm4_1 = imm4_1_11 & 0b11110;
                let imm11 = imm4_1_11 & 0b00001;
                let imm12 = imm12_10_5 & 0b1000000;
                let imm10_5 = imm12_10_5 & 0b0111111;

                let imm = (((imm4_1 | (imm12_10_5 << 5) | (imm11 << 11) | (imm12 << 6)) as i32)
                    << 19)
                    >> 19;

                match (opcode, funct3) {
                    (0b1100011, 0b000) => Instruction::BEQ {
                        rs1: rs1.into(),
                        rs2: rs2.into(),
                        imm,
                    },
                    (0b1100011, 0b001) => Instruction::BNE {
                        rs1: rs1.into(),
                        rs2: rs2.into(),
                        imm,
                    },
                    (0b1100011, 0b100) => Instruction::BLT {
                        rs1: rs1.into(),
                        rs2: rs2.into(),
                        imm,
                    },
                    (0b1100011, 0b101) => Instruction::BGE {
                        rs1: rs1.into(),
                        rs2: rs2.into(),
                        imm,
                    },
                    (0b1100011, 0b110) => Instruction::BLTU {
                        rs1: rs1.into(),
                        rs2: rs2.into(),
                        imm: imm as u32,
                    },
                    (0b1100011, 0b111) => Instruction::BGEU {
                        rs1: rs1.into(),
                        rs2: rs2.into(),
                        imm: imm as u32,
                    },
                    _ => Instruction::Undifined(inst),
                }
            }
            InstructionType::U => {
                let rd = (inst & masks::RD_MASK) >> 7;
                let imm32_12 = inst & masks::IMM31_12_MASK;

                let imm = imm32_12 as i32;

                match opcode {
                    0b0110111 => Instruction::LUI { rd: rd.into(), imm },
                    0b0010111 => Instruction::AUIPC { rd: rd.into(), imm },
                    _ => Instruction::Undifined(inst),
                }
            }
            InstructionType::J => {
                let rd = (inst & masks::RD_MASK) >> 7;
                let imm20_10_1_11_19_12 = (inst & masks::IMM31_12_MASK) >> 12;

                let imm20 = (imm20_10_1_11_19_12 & 0b1000_0000_0000_0000_0000) << 1;
                let imm10_1 = (imm20_10_1_11_19_12 & 0b0111_1111_1110_0000_0000) >> 8;
                let imm11 = (imm20_10_1_11_19_12 & 0b0000_0000_0001_0000_0000) << 3;
                let imm12_19 = (imm20_10_1_11_19_12 & 0b0000_0000_0000_1111_1111) << 12;

                let imm = (imm10_1 | imm11 | imm12_19 | imm20) as i32;

                match opcode {
                    0b1101111 => Instruction::JAL { rd: rd.into(), imm },
                    _ => Instruction::Undifined(inst),
                }
            }
        }
    } else {
        Instruction::Undifined(inst)
    }
}

fn get_type_and_opcode(inst: u32) -> Option<(InstructionType, u32)> {
    let opcode = inst & masks::TYPE_MASK;
    Some((TYPE_MAP[opcode as usize]?, opcode))
}

mod masks {
    pub(super) const TYPE_MASK: u32 = 0b0000_0000_0000_0000_0000_0000_0111_1111;

    pub(super) const RD_MASK: u32 = 0b0000_0000_0000_0000_0000_1111_1000_0000;
    pub(super) const FUNCT3_MASK: u32 = 0b0000_0000_0000_0000_0111_0000_0000_0000;
    pub(super) const RS1_MASK: u32 = 0b0000_0000_0000_1111_1000_0000_0000_0000;
    pub(super) const RS2_MASK: u32 = 0b0000_0001_1111_0000_0000_0000_0000_0000;
    pub(super) const FUNCT7_MASK: u32 = 0b1111_1110_0000_0000_0000_0000_0000_0000;
    pub(super) const IMM11_5_MASK: u32 = 0b1111_1111_0000_0000_0000_0000_0000_0000;
    pub(super) const IMM11_0_MASK: u32 = 0b1111_1111_1111_0000_0000_0000_0000_0000;
    pub(super) const IMM4_0_MASK: u32 = RD_MASK;
    pub(super) const IMM31_12_MASK: u32 = 0b1111_1111_1111_1111_1111_0000_0000_0000;
}

#[rustfmt::skip]
const TYPE_MAP: [Option<InstructionType>; 128] = [
    /*0b0000000 */ None,
    /*0b0000001 */ None,
    /*0b0000010 */ None,
    /*0b0000011 */ Some(InstructionType::I),
    /*0b0000100 */ None,
    /*0b0000101 */ None,
    /*0b0000110 */ None,
    /*0b0000111 */ None,
    /*0b0001000 */ None,
    /*0b0001001 */ None,
    /*0b0001010 */ None,
    /*0b0001011 */ None,
    /*0b0001100 */ None,
    /*0b0001101 */ None,
    /*0b0001110 */ None,
    /*0b0001111 */ Some(InstructionType::I),
    /*0b0010000 */ None,
    /*0b0010001 */ None,
    /*0b0010010 */ None,
    /*0b0010011 */ Some(InstructionType::I),
    /*0b0010100 */ None,
    /*0b0010101 */ None,
    /*0b0010110 */ None,
    /*0b0010111 */ Some(InstructionType::U),
    /*0b0011000 */ None,
    /*0b0011001 */ None,
    /*0b0011010 */ None,
    /*0b0011011 */ Some(InstructionType::I),
    /*0b0011100 */ None,
    /*0b0011101 */ None,
    /*0b0011110 */ None,
    /*0b0011111 */ None,
    /*0b0100000 */ None,
    /*0b0100001 */ None,
    /*0b0100010 */ None,
    /*0b0100011 */ Some(InstructionType::S),
    /*0b0100100 */ None,
    /*0b0100101 */ None,
    /*0b0100110 */ None,
    /*0b0100111 */ None,
    /*0b0101000 */ None,
    /*0b0101001 */ None,
    /*0b0101010 */ None,
    /*0b0101011 */ None,
    /*0b0101100 */ None,
    /*0b0101101 */ None,
    /*0b0101110 */ None,
    /*0b0101111 */ None,
    /*0b0110000 */ None,
    /*0b0110001 */ None,
    /*0b0110010 */ None,
    /*0b0110011 */ Some(InstructionType::R),
    /*0b0110100 */ None,
    /*0b0110101 */ None,
    /*0b0110110 */ None,
    /*0b0110111 */ Some(InstructionType::U),
    /*0b0111000 */ None,
    /*0b0111001 */ None,
    /*0b0111010 */ None,
    /*0b0111011 */ Some(InstructionType::R),
    /*0b0111100 */ None,
    /*0b0111101 */ None,
    /*0b0111110 */ None,
    /*0b0111111 */ None,
    /*0b1000000 */ None,
    /*0b1000001 */ None,
    /*0b1000010 */ None,
    /*0b1000011 */ None,
    /*0b1000100 */ None,
    /*0b1000101 */ None,
    /*0b1000110 */ None,
    /*0b1000111 */ None,
    /*0b1001000 */ None,
    /*0b1001001 */ None,
    /*0b1001010 */ None,
    /*0b1001011 */ None,
    /*0b1001100 */ None,
    /*0b1001101 */ None,
    /*0b1001110 */ None,
    /*0b1001111 */ None,
    /*0b1010000 */ None,
    /*0b1010001 */ None,
    /*0b1010010 */ None,
    /*0b1010011 */ None,
    /*0b1010100 */ None,
    /*0b1010101 */ None,
    /*0b1010110 */ None,
    /*0b1010111 */ None,
    /*0b1011000 */ None,
    /*0b1011001 */ None,
    /*0b1011010 */ None,
    /*0b1011011 */ None,
    /*0b1011100 */ None,
    /*0b1011101 */ None,
    /*0b1011110 */ None,
    /*0b1011111 */ None,
    /*0b1100000 */ None,
    /*0b1100001 */ None,
    /*0b1100010 */ None,
    /*0b1100011 */ Some(InstructionType::B),
    /*0b1100100 */ None,
    /*0b1100101 */ None,
    /*0b1100110 */ None,
    /*0b1100111 */ Some(InstructionType::I),
    /*0b1101000 */ None,
    /*0b1101001 */ None,
    /*0b1101010 */ None,
    /*0b1101011 */ None,
    /*0b1101100 */ None,
    /*0b1101101 */ None,
    /*0b1101110 */ None,
    /*0b1101111 */ Some(InstructionType::J),
    /*0b1110000 */ None,
    /*0b1110001 */ None,
    /*0b1110010 */ None,
    /*0b1110011 */ Some(InstructionType::I),
    /*0b1110100 */ None,
    /*0b1110101 */ None,
    /*0b1110110 */ None,
    /*0b1110111 */ None,
    /*0b1111000 */ None,
    /*0b1111001 */ None,
    /*0b1111010 */ None,
    /*0b1111011 */ None,
    /*0b1111100 */ None,
    /*0b1111101 */ None,
    /*0b1111110 */ None, 
    /*0b1111111 */ None,
];
