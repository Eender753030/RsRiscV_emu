use super::{Registers, PC};
use crate::device::bus::SystemBus; 
use crate::error::RiscVError;
use crate::isa::{decoder, Instruction, types::*};
use crate::engine::*;

const INIT_RAM_START: u32 = 0x8000_0000;

pub struct Cpu {
    reg: Registers,
    pc: PC,
    bus: SystemBus
}

impl Cpu {
    pub fn new(ram_size: usize) -> Self {
        let start_pointer = if ram_size > INIT_RAM_START as usize {
            INIT_RAM_START
        } else {
            0x0
        };

        Cpu {
            bus: SystemBus::new(ram_size),
            pc: PC::new(start_pointer),
            ..Default::default()
        }
    }

    pub fn load(&mut self, code: &[u8]) -> Result<(), RiscVError> {
        self.bus.load_ins(self.pc.get(), code)
    }

    pub fn cycle(&mut self) -> Result<(), RiscVError> {
        loop {
            if let Err(e) = self.step() {
                break match e {
                    RiscVError::EndOfInstruction => Ok(()),
                    e => Err(e)
                }
            }
        }
    }

    pub fn step(&mut self) -> Result<(), RiscVError> {   
        let instruction = self.fetch()?;
        if instruction == 0 {
            return Err(RiscVError::EndOfInstruction);
        }
        
        let type_data = self.decode(instruction)?;
        
        self.execute(type_data)?;
        Ok(())
    }

    fn fetch(&mut self) -> Result<u32, RiscVError> {
        self.bus.ram_fetch(self.pc.get())
    }

    fn decode(&self, bytes: u32) -> Result<Instruction, RiscVError> {
        decoder::decode(bytes)
    }

    fn execute(&mut self, ins: Instruction) -> Result<(), RiscVError> {
        match ins {
            Instruction::Itype(Itype {rd, rs1, funct3, imm}) => {
                let data = self.reg.read(rs1)?;
                self.reg.write(rd, alu::itype(data, imm, funct3)?)?;
            },
            Instruction::ItypeLoad(Itype {rd, rs1, funct3, imm}) => {
                let src = self.reg.read(rs1)?;
                let data = lsu::load(&mut self.bus, src, imm, funct3)?;
                self.reg.write(rd, data)?;
            },
            Instruction::ItypeJump(Itype{rd, rs1, imm, ..}) => {
                self.reg.write(rd, self.pc.get() + 4)?;
                self.pc.directed_addressing(self.reg.read(rs1)?.wrapping_add_signed(imm) & !1);
                return Ok(());
            },
            Instruction::Rtype(Rtype {rd, rs1, rs2, funct3, funct7}) => {
                let data1 = self.reg.read(rs1)?;
                let data2 = self.reg.read(rs2)?;
                self.reg.write(rd, alu::rtype(data1, data2, funct3, funct7)?)?;
            },
            Instruction::Stype(Stype {rs1, rs2, funct3, imm}) => {
                let src = self.reg.read(rs2)?;
                let des = self.reg.read(rs1)?;
                lsu::store(&mut self.bus, des, src, imm, funct3)?;
            },
            Instruction::Btype(Btype {rs1, rs2, funct3, imm}) => {
                let data1 = self.reg.read(rs1)?;
                let data2 = self.reg.read(rs2)?;
                if branch::branch(data1, data2, funct3)? {
                    self.pc.related_addressing(imm);
                    return Ok(());
                };
            },
            Instruction::UtypeLUI(Utype {rd, imm}) => {
                self.reg.write(rd, imm)?;
            },
            Instruction::UtypeAUIPC(Utype {rd, imm}) => {
                self.reg.write(rd, self.pc.get() + imm)?;
            },
            Instruction::Jtype(Jtype{rd, imm}) => {
                self.reg.write(rd, self.pc.get() + 4)?;
                self.pc.related_addressing(imm);
                return Ok(());
            },
            Instruction::ItypeSys(Itype {imm, ..}) => {
                if imm == 0 {
                    let sys_call_id = self.reg.read(17)?;
                    sys::syscall(&mut self.bus, &self.reg, sys_call_id)?;
                }
            }
        }

        self.pc.step();
        Ok(())
    }

    pub fn reset(&mut self) {
        self.bus.ram.reset();
        self.pc.reset(INIT_RAM_START);
        self.reg.reset();
    }

    pub fn dump_data(&self) -> (Vec<i32>, Vec<[u8; 4]>, u32) {
        (
            self.reg.dump(),
            vec![[0;4]; 10], // Give 0 to let compile pass
            self.pc.get()
        )
    }
}

impl Default for Cpu {
    fn default() -> Self {
        Cpu {
            reg: Registers::default(),
            pc: PC::new(INIT_RAM_START),
            bus: SystemBus::default(),
        }
    }
}