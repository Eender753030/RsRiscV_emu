use crate::device::Device;

pub struct Uart {
    thr: u32,
    lsr: u32,
}

impl Device for Uart {
    fn read_byte(&self, addr: u32) -> Result<u8, crate::prelude::Exception> {
        
    }

    fn write_byte(&mut self, addr: u32, data: u8) -> Result<(), crate::prelude::Exception> {
        
    }

    fn read_bytes(&self, addr: u32, size: usize, des: &mut [u8]) -> Result<(), crate::prelude::Exception> {
        
    }

    fn write_bytes(&mut self, addr: u32, size: usize, src: &[u8]) -> Result<(), crate::prelude::Exception> {
        
    }
}