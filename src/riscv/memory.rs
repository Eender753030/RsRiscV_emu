use crate::utils::exception::RiscVError;

pub struct Memory {
    size: usize,
    space: Vec<u8>,
}

impl Memory {
    pub fn new(size: usize) -> Self {
        Memory{size, space: vec![0; size]}
    }
    
    pub fn read(&self, pc: u32) -> Result<u32, RiscVError> {
        let idx = pc as usize;

        if idx >= self.size {
            return Err(RiscVError::OutOfBoundMemory);
        }

        let mut read_data:u32 = self.space[idx + 3] as u32;


        for i in 1..4 {
            read_data <<= 8;
            read_data |= self.space[idx + 3 - i] as u32;
        }    

        println!("{:08x}", read_data);

        Ok(read_data)
    } 

    pub fn write(&mut self, pc: u32, data: u32) -> Result<(), RiscVError> {
        let idx = pc as usize;

        if idx >= self.size {
            return Err(RiscVError::OutOfBoundMemory);
        }

        let mut write_data = data;
        for i in 0..4 {
            self.space[idx + i] = (write_data & 0xff) as u8;
            write_data >>= 8;
        }
        
        Ok(())
    } 

}