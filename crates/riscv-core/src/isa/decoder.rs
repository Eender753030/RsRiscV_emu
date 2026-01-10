use super::instruction::Instruction;
use super::opcode::{OpCode, Bits};
use crate::error::RiscVError;
use crate::isa::types::*;

/// Turn 32 bits instruction to corresponding `Instruction` enum
/// May return `RiscVError` of `NotImplementedOpCode`
/// 
/// ## Example
/// ```rust
/// # use riscv_core::prelude::*;
/// # use decoder::decode;
/// # use instruction::Instruction;
/// # use types::Rtype;
/// // add x5, x6, x7
/// # fn main() {
/// let ins: u32 = 0x007302b3;
/// let execpt = Instruction::Rtype(Rtype {
///     rd: 5, rs1: 6, rs2: 7, funct3: 0, funct7: 0
/// });
/// 
/// assert_eq!(decode(ins), Ok(execpt));
/// assert_eq!(decode(0x01), Err(RiscVError::NotImplementedOpCode(0x01)));
/// # }
/// ```
pub fn decode(ins: u32) -> Result<Instruction, RiscVError> {
    let opcode = (ins & Bits::_6_TO_0) as u8;
    let rd = ((ins & Bits::_11_TO_7) >> 7) as u8;
    let rs1 = ((ins & Bits::_19_TO_15) >> 15) as u8;
    let rs2 = ((ins & Bits::_24_TO_20) >> 20) as u8;
    let funct3 = ((ins & Bits::_14_TO_12) >> 12) as u8;
    let funct7 = ((ins & Bits::_31_TO_25) >> 25) as u8;
    
    Ok(match opcode {
        // imm [11:0] | rs1 [4:0] | funct3 [2:0] | rd [4:0] | opcode [6:0]
        OpCode::ITYPE => {
            let imm = ((ins & Bits::_31_TO_20) as i32) >> 20;
            Instruction::Itype(Itype {
                rd, rs1, funct3, imm
            })
        },
        // imm [11:0] | rs1 [4:0] | funct3 [2:0] | rd [4:0] | opcode [6:0]
        OpCode::ITYPE_LOAD => {
            let imm = ((ins & Bits::_31_TO_20) as i32) >> 20;
            Instruction::ItypeLoad(Itype {
                rd, rs1, imm, funct3
            })
        },
        // imm [11:0] | rs1 [4:0] | funct3 [2:0] | rd [4:0] | opcode [6:0]
        OpCode::ITYPE_JUMP => {
            let imm = ((ins & Bits::_31_TO_20) as i32) >> 20;
            Instruction::ItypeJump(Itype {
                rd, rs1, imm, funct3
            })
        },
        // imm [11:0] | rs1 [4:0] | funct3 [2:0] | rd [4:0] | opcode [6:0]
        OpCode::ITYPE_SYS => {
            let imm = ((ins & Bits::_31_TO_20) as i32) >> 20;
            Instruction::ItypeSys(Itype {
                rd, rs1, imm, funct3
            })
        },
        // funct7 [6:0] | rs2 [4:0] | rs1 [4:0] | funct3 [2:0]  | rd [4:0] | opcode [6:0]
        OpCode::RTYPE => {
            Instruction::Rtype(Rtype {
                rd, rs1, rs2, funct3, funct7
            })
        },
        // imm [11:5] | rs2 [4:0] | rs1 [4:0] | funct3 [2:0] | imm [4:0] | opcode [6:0]
        OpCode::STYPE => {
            let imm = (((ins & Bits::_31_TO_25) as i32) >> 20) | (((ins & Bits::_11_TO_7) >> 7) as i32);
            Instruction::Stype(Stype {
                rs1, rs2, imm, funct3
            })
        },
        // imm[12|10:5] | rs2 [4:0] | rs1 [4:0] | funct3 [2:0] | imm[4:1|11] | opcode [6:0]
        OpCode::BTYPE => {
            let imm = (((ins & Bits::_31) as i32) >> 19) | ((((ins & Bits::_7) << 4) | ((ins & Bits::_30_TO_25) >> 20) | ((ins & Bits::_11_TO_8) >> 7)) as i32);
            Instruction::Btype(Btype {
                rs1, rs2, imm, funct3
            })
        },
        // imm[20|10:1|11|19:12] | rd[4:0] | opcode[6:0]
        OpCode::JTYPE => {
            let imm = (((ins & Bits::_31) as i32) >> 11) | (((ins & Bits::_19_TO_12) | ((ins & Bits::_20) >> 9) | ((ins & Bits::_30_TO_21) >> 20)) as i32);
            Instruction::Jtype(Jtype {
                rd, imm
            })
        },
        // imm[20|10:1|11|19:12] | rd[4:0] | opcode[6:0]
        OpCode::UTYPE_LUI => {
            let imm = ins & Bits::_31_TO_12; 
            Instruction::UtypeLUI(Utype { 
                rd, imm
            })
        },
        // imm[20|10:1|11|19:12] | rd[4:0] | opcode[6:0]
        OpCode::UTYPE_AUIPC => {
            let imm = ins & Bits::_31_TO_12; 
            Instruction::UtypeAUIPC(Utype { 
                rd, imm
            })  
        },  
        illegal => {
            return Err(RiscVError::NotImplementedOpCode(illegal));
        }
    })
}

#[cfg(test)]
mod decoder_test {
    use crate::isa::instruction::Instruction;
    use crate::isa::{decoder, types::*};
    use crate::error::RiscVError;

    #[test]
    // Test I-type instruction
    fn itype_test() {
        // addi x10, x0, -2048
        let ins1 = 0x80000513;
        let except1 = Instruction::Itype(Itype {
            rd: 10, rs1: 0, funct3: 0, imm: -2048
        });
        // slti x23, x24, 31
        let ins2 = 0x01fc2b93;
        let except2 = Instruction::Itype(Itype {
            rd: 23, rs1: 24, funct3: 2, imm: 31
        });
        // andi x31, x22, 2047
        let ins3 = 0x7ffb7f93;
        let except3 = Instruction::Itype(Itype {
            rd: 31, rs1: 22, funct3: 7, imm: 2047
        });

        assert_eq!(decoder::decode(ins1), Ok(except1));

        assert_eq!(decoder::decode(ins2), Ok(except2));

        assert_eq!(decoder::decode(ins3), Ok(except3));      
    }

    #[test]
    // Test I-type Load instruction
    fn itype_load_test() {
        // lw x5, 12(x7)
        let ins1 = 0x00c3a283;
        let except1 = Instruction::ItypeLoad(Itype {
            rd: 5, rs1: 7, funct3: 2, imm: 12
        });
        // lbu x19, 0(x11) 
        let ins2 = 0x0005c983;
        let except2 = Instruction::ItypeLoad(Itype {
            rd: 19, rs1: 11, funct3: 4, imm: 0
        });

        assert_eq!(decoder::decode(ins1), Ok(except1));

        assert_eq!(decoder::decode(ins2), Ok(except2));
    }

    #[test]
    // Test I-type Jump instruction
    fn itype_jump_test() {
        // jalr x0, 256(x28)
        let ins1 = 0x100e0067;
        let except1 = Instruction::ItypeJump(Itype {
            rd: 0, rs1: 28, funct3: 0, imm: 256
        });
        // jalr x1, -442(x21)
        let ins2 = 0xe46a80e7;
        let except2 = Instruction::ItypeJump(Itype {
            rd: 1, rs1: 21, funct3: 0, imm: -442
        });

        assert_eq!(decoder::decode(ins1), Ok(except1));

        assert_eq!(decoder::decode(ins2), Ok(except2));
    }

    #[test]
    // Test I-type Jump instruction
    fn itype_sys_test() {
        // ecall
        let ins1 = 0x00000073;
        let except1 = Instruction::ItypeSys(Itype {
            rd: 0, rs1: 0, funct3: 0, imm: 0
        });
        // ebreak
        let ins2 = 0x00100073;
        let except2 = Instruction::ItypeSys(Itype {
            rd: 0, rs1: 0, funct3: 0, imm: 1
        });

        assert_eq!(decoder::decode(ins1), Ok(except1));

        assert_eq!(decoder::decode(ins2), Ok(except2));
    }

    #[test]
    // Test I-type Jump instruction
    fn rtype_test() {
        // sub x13, x15, x18
        let ins1 = 0x412786b3;
        let except1 = Instruction::Rtype(Rtype {
            rd: 13, rs1: 15, rs2: 18, funct3: 0, funct7: 32
        });
        // sll x2, x6, x9
        let ins2 = 0x00931133;
        let except2 = Instruction::Rtype(Rtype {
            rd: 2, rs1: 6, rs2: 9, funct3: 1, funct7: 0
        });
        // xor x20, x30, x26
        let ins3 = 0x01af4a33;
        let except3 = Instruction::Rtype(Rtype {
            rd: 20, rs1: 30, rs2: 26, funct3: 4, funct7: 0
        });

        assert_eq!(decoder::decode(ins1), Ok(except1));

        assert_eq!(decoder::decode(ins2), Ok(except2));

        assert_eq!(decoder::decode(ins3), Ok(except3));      
    }

    #[test]
    // Test I-type Jump instruction
    fn stype_test() {
        // sw x21, 123(x13)
        let ins1 = 0x0756ada3;
        let except1 = Instruction::Stype(Stype {
            rs1: 13, rs2: 21, funct3: 2, imm: 123 
        });
        // sh x11, -567(x22)
        let ins2 = 0xdcbb14a3;
        let except2 = Instruction::Stype(Stype {
            rs1: 22, rs2: 11, funct3: 1, imm: -567
        });

        assert_eq!(decoder::decode(ins1), Ok(except1));

        assert_eq!(decoder::decode(ins2), Ok(except2));
    }

    #[test]
    // Test I-type Jump instruction
    fn jtype_test() {
        // jal x1, 140
        let ins1 = 0x046000ef;
        let except1 = Instruction::Jtype(Jtype {
            rd: 1, imm: 70
        });
        // jal x0, -32
        let ins2 = 0xff1ff06f;
        let except2 = Instruction::Jtype(Jtype {
            rd: 0, imm: -16
        });

        assert_eq!(decoder::decode(ins1), Ok(except1));

        assert_eq!(decoder::decode(ins2), Ok(except2));
    }

    #[test]
    // Test I-type Jump instruction
    fn utype_test() {
        // auipc x29, 0x100
        let ins1 = 0x00100e97;
        let except1 = Instruction::UtypeAUIPC(Utype {
            rd: 29, imm: 0x100_000
        });
        // lui x19, 0x20000
        let ins2 = 0x200009b7;
        let except2 = Instruction::UtypeLUI(Utype {
            rd: 19, imm: 0x20000_000 
        });

        assert_eq!(decoder::decode(ins1), Ok(except1));

        assert_eq!(decoder::decode(ins2), Ok(except2));
    }

    #[test]
    // Test illegal opcode
    fn err_test() { 
        let err_ins1 = 0xffffffff;
        let err_ins2 = 0x0;

        assert_eq!(decoder::decode(err_ins1), Err(RiscVError::NotImplementedOpCode(0x7f)));
        assert_eq!(decoder::decode(err_ins2), Err(RiscVError::NotImplementedOpCode(0x0)));
    }
}