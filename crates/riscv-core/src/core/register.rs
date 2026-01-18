use crate::error::RiscVError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Registers {
    reg: [u32; 32],
} 

impl Registers {
    pub fn read(&self, id: u8) -> Result<u32, RiscVError> {
        if id == 0 {
            return Ok(0);
        } 

        if id > 31 {
            return Err(RiscVError::InvalidRegister(id));
        } 

        Ok(self.reg[id as usize])
    }

    pub fn write(&mut self, id: u8, data: u32) -> Result<(), RiscVError> {
        if id == 0 {
            return Ok(());
        }

        if id > 31 {
            return Err(RiscVError::InvalidRegister(id));
        } 

        self.reg[id as usize] = data;


        Ok(())
    } 
    
    pub fn reset(&mut self) {
        self.reg.fill(0);
    }

    pub fn dump(&self) -> Vec<i32> {
        self.reg.iter().map(|&x| x as i32).collect()
    }
}