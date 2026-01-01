use crate::riscv::{Instruction, InstructionKind};

pub fn parse_binary(binary_content: Vec<u8>) -> Vec<String> {
    let mut instructions = Vec::new();

    for binary_ins in binary_content.chunks(4) {
        let binary = u32::from_le_bytes(binary_ins.try_into().unwrap());
        let ins_type = get_ins_type(binary);
        instructions.push(get_ins_string(ins_type, instructions.len()));
    }

    instructions
}

fn get_ins_type(ins: u32) -> Instruction {
    match ins & 0x7f {
        0x03 => Instruction::parse(InstructionKind::ItypeLoad, ins),
        
        0x13 => Instruction::parse(InstructionKind::Itype, ins),
        
        0x17 => Instruction::parse(InstructionKind::UtypeAUIPC, ins),

        0x23 => Instruction::parse(InstructionKind::Stype, ins),

        0x33 => Instruction::parse(InstructionKind::Rtype, ins),

        0x37 => Instruction::parse(InstructionKind::UtypeLUI, ins),

        0x63 => Instruction::parse(InstructionKind::Btype, ins),

        0x67 => Instruction::parse(InstructionKind::ItypeJump, ins),

        0x6f => Instruction::parse(InstructionKind::Jtype, ins),

        0x73 => Instruction::parse(InstructionKind::ItypeSys, ins),

        // No use
        _ => Instruction::parse(InstructionKind::ItypeSys, ins)
    }
}

fn get_ins_string(ins_type: Instruction, ins_count: usize) -> String {
    let mut ins_line = String::new();
    match ins_type {
        Instruction::Itype {rd, rs1, imm, funct3} => {  
            ins_line.push_str(
                match funct3 {
                    0x0 => "addi   ",               
                    0x1 => "slli   ",
                    0x2 => "slti   ",
                    0x3 => "sltiu  ",
                    0x4 => "xori   ",  
                    0x5 => {
                        match (imm & 0xfe0) >> 5 {
                            0x00 => "srli   ",
                            0x20 => "srai   ",
                            _ => " "
                        }
                    },
                    0x6 => "ori    ",
                    0x7 => "andi   ",
                    _ => " "
                }
            );
            ins_line.push_str(&format!(" x{}, x{}, {}", rd, rs1, imm));
        },

        Instruction::ItypeLoad {rd, rs1, imm, funct3} => {
            ins_line.push_str(
                match funct3 {
                    0x0 => "lb     ",
                    0x1 => "lh     ",
                    0x2 => "lw     ",
                    0x4 => "lbu    ",
                    0x5 => "lhu    ",
                    _ => " "
                }
            );      
            ins_line.push_str(&format!(" x{}, {}(x{})", rd, imm, rs1));
        },

        Instruction::Rtype {rd, rs1, rs2, funct3, funct7} => {
            ins_line.push_str(
                match funct3 {
                    0x0 => {             
                        match funct7 {
                            0x00 => "add    ",
                            0x20 => "sub    ",
                            _ => " "
                        }                  
                    },
                    0x1 => "sll    ",
                    0x2 => "slt    ",
                    0x3 => "sltu   ",
                    0x4 => "xor    ",
                    0x5 => {
                        match funct7 {
                            0x00 => "srl    ",
                            0x20 => "sra    ",
                            _ => " "
                        }   
                    },
                    0x6 => "or     ",
                    0x7 => "and    ",
                    _ => " "
                }
            );
            ins_line.push_str(&format!(" x{}, x{}, x{}", rd, rs1, rs2));
        },

        Instruction::Stype {rs1, rs2, imm, funct3} => {
            ins_line.push_str(
                match funct3 {
                    0x0 => "sb     ", // SB
                    0x1 => "sh     ", // SH
                    0x2 => "sw     ", // SW
                    _ => " "
                }
            );
            ins_line.push_str(&format!(" x{}, {}(x{})", rs2, imm, rs1));
        },

        Instruction::Btype {rs1, rs2, imm, funct3} => { 
            ins_line.push_str(
                match funct3 {
                    0x0 => "beq    ",
                    0x1 => "bne    ",
                    0x4 => "blt    ",
                    0x5 => "bge    ",
                    0x6 => "bltu   ",
                    0x7 => "bgeu   ",
                    _ => " "
                }
            );
            ins_line.push_str(&format!(" x{}, x{}, {}    # Go to {}.", rs1, rs2, imm << 1, (ins_count as i32) + (imm >> 1) + 1));
        }

        Instruction::UtypeLUI {rd, imm} => {
            ins_line.push_str(&format!("lui     x{}, 0x{:x}", rd, imm >> 12));
        },

        Instruction::UtypeAUIPC {rd, imm} => {
            ins_line.push_str(&format!("auipc   x{}, {}", rd, imm));
        },

        Instruction::Jtype {rd, imm} => {
            ins_line.push_str(&format!("jal     x{}, {}      # Go to {}.", rd, imm << 1, (ins_count as i32) + (imm >> 1) + 1));
        },

        Instruction::ItypeJump {rd, rs1, imm} => {
            ins_line.push_str(&format!("jalr    x{}, {}(x{})    # Go to {}.", rd, imm, rs1, (ins_count as i32) + (imm >> 1) + 1));
        },

        Instruction::ItypeSys {imm} => {
            if imm == 0 {
                ins_line.push_str("ecall");
            } else {
                ins_line.push_str("ebreak");
            }
        }
    }
    ins_line
}