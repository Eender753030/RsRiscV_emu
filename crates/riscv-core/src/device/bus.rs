use crate::exception::Exception;
use crate::core::Access;

use super::Device;
use super::memory::{Memory, PAGE_SIZE};
use super::uart::Uart;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct SystemBus {
    uart: Uart,
    ram: Memory,
}

pub const UART_BASE: u32 = 0x1000_0000;
pub const UART_END: u32 = 0x1000_00FF;
pub const DRAM_BASE_ADDR: u32 = 0x8000_0000;

impl SystemBus {
    pub fn mapping(&self, assess: &mut Access) -> Result<&dyn Device, Exception> {
        let addr = assess.addr;
        match addr {
            UART_BASE..=UART_END => {
               assess.addr = addr - UART_BASE;
               Ok(&self.uart)
            }
            DRAM_BASE_ADDR.. => {
                let ram_addr = addr - DRAM_BASE_ADDR;
                if ram_addr as usize >= self.ram.size {
                    Err(assess.to_access_exception())
                } else {
                    assess.addr = ram_addr;
                    Ok(&self.ram)
                }
            },
            _ => Err(assess.to_access_exception()),
        }
    }

    pub fn mapping_mut(&mut self, assess: &mut Access) -> Result<&mut dyn Device, Exception> {
        let addr = assess.addr;
        match addr {
            UART_BASE..=UART_END => {
               assess.addr = addr - UART_BASE;
               Ok(&mut self.uart)
            }
            DRAM_BASE_ADDR.. => {
                let ram_addr = addr - DRAM_BASE_ADDR;
                if ram_addr as usize >= self.ram.size {
                    Err(assess.to_access_exception())
                } else {
                    assess.addr = ram_addr;
                    Ok(&mut self.ram)
                }
            },
            _ => Err(assess.to_access_exception()),
        }
    }

    pub fn read_u32(&self, assess: Access) -> Result<u32, Exception> { 
        self.read_u32_bytes(assess, 4, false)
    }

    pub fn read_u32_bytes(&self, mut assess: Access, len: usize, is_signed: bool) -> Result<u32, Exception> {
        let device = self.mapping(&mut assess)?;

        let mut four_bytes = [0; 4];

        device.read_bytes(assess, len, &mut four_bytes[..len])?;

        if is_signed && (four_bytes[len - 1] & 0x80 != 0) {
            four_bytes[len..].fill(0xff);
        }

        Ok(u32::from_le_bytes(four_bytes))
    }

    pub fn write_u32(&mut self, assess: Access, data: u32) -> Result<(), Exception> {
        self.write_u32_bytes(assess, data, 4)
    }

    pub fn write_u32_bytes(&mut self, mut assess: Access, data: u32, len: usize) -> Result<(), Exception> {
        let device = self.mapping_mut(&mut assess)?;
        device.write_bytes(assess, len, &data.to_le_bytes())?;
        Ok(())
    }

    pub fn ram_info(&self) -> (usize, u32, usize) {
        (self.ram.size, DRAM_BASE_ADDR, PAGE_SIZE)
    }

    pub fn reset_ram(&mut self) {
        self.ram.reset();
    }
}

impl Device for SystemBus {
    fn read_byte(&self, mut assess: Access) -> Result<u8, Exception> {
        let device = self.mapping(&mut assess)?;
        device.read_byte(assess)
    }

    fn write_byte(&mut self, mut assess: Access, data: u8) -> Result<(), Exception> {
        let device = self.mapping_mut(&mut assess)?;
        device.write_byte(assess, data)
    }

    fn read_bytes(&self, mut assess: Access, size: usize, des: &mut [u8]) -> Result<(), Exception> {
        let device = self.mapping(&mut assess)?;
        device.read_bytes(assess, size, des)
    }

    fn write_bytes(&mut self, mut assess: Access, size: usize, src: &[u8]) -> Result<(), Exception> {
        let device = self.mapping_mut(&mut assess)?;
        device.write_bytes(assess, size, src)
    }
}
