mod register;
mod pc;
mod memory;
mod instruction;
pub mod loader;

use std::ops::Shr;

use register::Registers;
use pc::PC;
use memory::Memory;
use crate::{riscv::instruction::{Instruction, InstructionKind}, utils::exception::RiscVError};

pub struct RiscV {
    registers: Registers,
    pc: PC,
    ins_memory: Memory,
    data_memory: Memory,
}

impl RiscV {
    pub fn new() -> Self {
        RiscV {
            registers: Registers::new(),
              pc: PC::new(),
            ins_memory: Memory::new(128),
            data_memory: Memory::new(512),
        }
    }
    
    pub fn cycle(&mut self, code: &[u8]) -> Result<(), RiscVError>{
        self.ins_memory.load(0, code)?;

        loop {
            let instruction = self.fetch()?;
            if instruction == 0 {
                break Ok(());
            }
            
            let type_data = self.decode(instruction)?;
            
            self.execute(type_data)?;
        }
    }

    fn fetch(&self) -> Result<u32, RiscVError> {
        self.ins_memory.fetch(self.pc.get())
    }

    fn decode(&self, instruction: u32) -> Result<Instruction, RiscVError>{
        match instruction & 0x7f {
            0x03 => Ok(Instruction::parse(InstructionKind::ItypeLoad, instruction)),
            
            0x13 => Ok(Instruction::parse(InstructionKind::Itype, instruction)),
            
            0x23 => Ok(Instruction::parse(InstructionKind::Stype, instruction)),

            0x33 => Ok(Instruction::parse(InstructionKind::Rtype, instruction)),
            
            not_exist_opcode => Err(RiscVError::NotImplementedOpCode(not_exist_opcode))
                    }
    }

    fn execute(&mut self, op_type: Types) -> Result<(), RiscVError> {
        match op_type {
            Types::IType {imm, rs1, rd, func} => {
                match func {
                    // ADDI

            Instruction::ItypeLoad {rd, rs1, imm, funct3} => {
                self.registers.write(
                    rd,
                    self.data_memory.read(
                        self.registers.read(rs1)?.wrapping_add_signed(imm),
                        match funct3 {
                            0x0 | 0x4 => 0, // LB | LBU
                            0x1 | 0x5 => 1, // LH | LHU
                            0x2 => 3, // LW
                            not_exist_funct => return Err(RiscVError::NotImplementedFunc(0x03, not_exist_funct))
                        },
                        match funct3 {
                            0x0 | 0x1 | 0x2 => true, // LB | LH | LW
                            0x4 | 0x5 => false, // LBU | LHU
                            not_exist_funct => return Err(RiscVError::NotImplementedFunc(0x03, not_exist_funct))
                        },
                    )?
                )?;      
            },
                    0x0 => { 

            Instruction::Stype {rs1, rs2, imm, funct3} => {
                self.data_memory.write(
                    self.registers.read(rs1)?.wrapping_add_signed(imm), 
                    self.registers.read(rs2)?, 
                    match funct3 {
                        0x0 => 0, // SB
                        0x1 => 1, // SH
                        0x2 => 3, // SW
                        not_exist_funct => return Err(RiscVError::NotImplementedFunc(0x23, not_exist_funct))
                    }
                )?;
            }
        }

        self.pc.step();
        Ok(())
    }

    pub fn print(&self) {
        println!("Registers {{ {:?} }}\n{:?}\n{:x?}", self.registers.dump_signed_vec(), self.pc, self.data_memory);
    }
}