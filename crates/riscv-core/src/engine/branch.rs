use crate::error::RiscVError;
use super::super::isa::opcode::OpCode;

pub fn branch(data1: u32, data2: u32, funct3: u8) -> Result<bool, RiscVError> {
        Ok(match funct3 {
            0x0 => data1 == data2, // BEQ
            0x1 => data1 != data2, // BNE
            0x4 => (data1 as i32) < (data2 as i32), // BLT (Signed)
            0x5 => (data1 as i32) >= (data2 as i32), // BGE (Signed)
            0x6 => data1 < data2, // BLTU
            0x7 => data1 >= data2, // BGEU
            not_exist_funct => return Err(RiscVError::NotImplementedFunc(OpCode::BTYPE, not_exist_funct))
        })
}