use riscv_decoder::prelude::*;

use super::{PC, RegisterFile, CsrFile, PrivilegeMode};
use crate::core::{Access, AccessType, Mmu};
use crate::device::bus::SystemBus;
use crate::device::Device;
use crate::error::RiscVError;
use crate::exception::Exception;
use crate::debug::*;

#[derive(Clone, PartialEq, Default)]
pub struct Cpu {
    pub(crate) mode: PrivilegeMode,
    pub(crate) regs: RegisterFile,
    pub(crate) pc: PC,
    pub(crate) csrs: CsrFile,
    pub(crate) bus: SystemBus,
}

impl Cpu {
    pub fn load(&mut self, addr: u32, data: &[u8]) -> Result<(), RiscVError> {
        let access = Access::new(addr, AccessType::Store);
        if self.bus.write_bytes(access, data.len(), data).is_err() {
            Err(RiscVError::LoadFailed)
        } else {
            Ok(())
        }
    }

    pub fn set_pc(&mut self, entry: u32) {
        self.pc.set(entry);
    }

    pub fn set_mem_zero(&mut self, addr: u32, size: usize) -> Result<(), RiscVError> {
        for i in 0..size {
            let access = Access::new(addr + i as u32, AccessType::Store);
            self.bus.write_byte(access, 0)
                .map_err(|_| RiscVError::BssInitFailed)?
        }
        Ok(())
    }

    pub fn run(&mut self) -> Result<(), RiscVError> {
        loop { self.step()? }
    }
 
    pub fn step(&mut self) -> Result<(), RiscVError> {
        if let Err(execpt) = self.cycle() {        
            self.trap_handle(execpt);
        }
        Ok(())
    }

    fn cycle(&mut self) -> Result<(), Exception> {
        let raw = self.fetch()?;
        
        let ins = self.decode(raw)?;
        
        self.execute(ins)?;
        Ok(())
    }

    fn fetch(&mut self) -> Result<u32, Exception> {
        let va_access = Access::new(self.pc.get(), super::AccessType::Fetch);

        let pa_access = Mmu::translate(va_access, self.mode, self.csrs.check_satp(), &mut self.bus)?;

        match self.bus.read_u32(pa_access) {
            Ok(raw) => Ok(raw),
            Err(e) => {
                match e {
                    Exception::LoadAccessFault(_) => Err(Exception::LoadAccessFault(va_access.addr)),
                    Exception::StoreAccessFault(_) => Err(Exception::StoreAccessFault(va_access.addr)),
                    _ => Err(e),
                }
            }
        }    
    }

    fn decode(&self, bytes: u32) -> Result<Instruction, Exception> {
        decoder::decode(bytes)
            .map_err(|_| Exception::IllegalInstruction(bytes))
    }

    fn execute(&mut self, ins: Instruction) -> Result<(), Exception> {
        match ins {
            Instruction::Base(op, data)  => if self.execute_rv32i(op, data)? {
                    return Ok(());
            },
            Instruction::Privileged(op)  => {
                self.execute_privileged(op);
                return Ok(())
            },
            Instruction::M(op, data)     => self.execute_m(op, data),
            Instruction::Ziscr(op, data) => self.execute_zicsr(op, data)?,
            Instruction::Zifencei(_, _)  => {},          
        }
        self.pc.step();
        Ok(())
    }

    fn trap_handle(&mut self, except: Exception) {
        let (mode, pc) = self.csrs.trap_entry(self.pc.get(), except, self.mode);
        self.pc.directed_addressing(pc);
        self.mode = mode;
    }

    pub fn reset(&mut self) {
        self.mode = PrivilegeMode::default();
        self.regs.reset();
        self.csrs.reset();
        self.pc.reset();
        self.bus.reset_ram();
    }
}

impl DebugInterface for Cpu {
    fn inspect_regs(&self) -> [u32; 32] {
        self.regs.inspect()
    }

    fn inspect_pc(&self) -> u32 {
        self.pc.get()
    }

    fn inspect_csrs(&self) -> Vec<(String, u32)> {
        self.csrs.inspect()
    }

    fn inspect_mem(&self, addr: u32, len: usize) -> Vec<u8> {
        let mut mem: Vec<u8> = vec![0; len]; 
        // Todo: The execption debuger layout
        let access = Access::new(addr, AccessType::Load);
        let _ = self.bus.read_bytes(access, len, &mut mem);
        mem
    }    

    fn get_info(&self) -> MachineInfo {
        let (dram_size, dram_base, page_size) = self.bus.ram_info();

        MachineInfo::new(dram_size, dram_base, page_size)
    }
}

impl std::fmt::Debug for Cpu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Cpu {{")?;
        writeln!(f, " PC: {:#08x}", self.pc.get())?;
        write!(f, " Registers {{")?;
        self.regs.iter().enumerate().try_for_each(|(id, regs)|
            write!(f, " x{}: {}", id, *regs as i32)
        )?;
        writeln!(f, " }}")?;
        write!(f, " {:?}", self.bus)
    }
}
