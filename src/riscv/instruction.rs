#[derive(Debug)]

pub enum InstructionKind {
    Itype,
    ItypeLoad,
    Rtype,
    Stype,
}

#[derive(Debug)]
pub enum Instruction {
    Itype {rd: usize, rs1: usize, imm: i32, funct3: u8},
    ItypeLoad {rd: usize, rs1: usize, imm: i32, funct3: u8},
    Rtype {rd: usize, rs1: usize, rs2: usize, funct3: u8, funct7: u8},
    Stype {rs1: usize, rs2: usize, imm: i32, funct3: u8},
}

impl Instruction {
    pub fn parse(ins_type: InstructionKind, ins: u32) -> Self {
        match ins_type {
            InstructionKind::Itype  => {
                Instruction::Itype {
                    rd:  ((ins >> 7) & 0x1f) as usize,
                    rs1: ((ins >> 15) & 0x1f) as usize,
                    imm: ((ins & 0xfff00000) as i32) >> 20,
                    funct3: ((ins >> 12) & 0x7) as u8
                }
            },

            InstructionKind::ItypeLoad  => {
                Instruction::ItypeLoad {
                    rd:  ((ins >> 7) & 0x1f) as usize,
                    rs1: ((ins >> 15) & 0x1f) as usize,
                    imm: ((ins & 0xfff00000) as i32) >> 20,
                    funct3: ((ins >> 12) & 0x7) as u8
                }
            },

            InstructionKind::Rtype => {
                Instruction::Rtype {
                    rd:  ((ins >> 7) & 0x1f) as usize,
                    rs1: ((ins >> 15) & 0x1f) as usize,
                    rs2: ((ins >> 20) & 0x1f) as usize,
                    funct3: ((ins >> 12) & 0x7) as u8,
                    funct7: ((ins >> 25) & 0x7f) as u8,
                }
            },

            InstructionKind::Stype => {
                Instruction::Stype {
                    rs1: ((ins >> 15) & 0x1f) as usize,
                    rs2: ((ins >> 20) & 0x1f) as usize,
                    imm: (((ins & 0xfe000000) as i32) >> 20) | (((ins >> 7) & 0x1f) as i32),
                    funct3: ((ins >> 12) & 0x7) as u8,
                }
            }
        }
    }
}