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
    pub fn mapping(&self, access: &mut Access) -> Result<&dyn Device, Exception> {
        let addr = access.addr;
        match addr {
            UART_BASE..=UART_END => {
               access.addr = addr - UART_BASE;
               Ok(&self.uart)
            }
            DRAM_BASE_ADDR.. => {
                let ram_addr = addr - DRAM_BASE_ADDR;
                if ram_addr as usize >= self.ram.size {
                    Err(access.to_access_exception())
                } else {
                    access.addr = ram_addr;
                    Ok(&self.ram)
                }
            },
            _ => Err(access.to_access_exception()),
        }
    }

    pub fn mapping_mut(&mut self, access: &mut Access) -> Result<&mut dyn Device, Exception> {
        let addr = access.addr;
        match addr {
            UART_BASE..=UART_END => {
               access.addr = addr - UART_BASE;
               Ok(&mut self.uart)
            }
            DRAM_BASE_ADDR.. => {
                let ram_addr = addr - DRAM_BASE_ADDR;
                if ram_addr as usize >= self.ram.size {
                    Err(access.to_access_exception())
                } else {
                    access.addr = ram_addr;
                    Ok(&mut self.ram)
                }
            },
            _ => Err(access.to_access_exception()),
        }
    }

    pub fn read_u32(&self, access: Access) -> Result<u32, Exception> { 
        self.read_u32_bytes(access, 4, false)
    }

    pub fn read_u32_bytes(&self, mut access: Access, len: usize, is_signed: bool) -> Result<u32, Exception> {
        let device = self.mapping(&mut access)?;

        let mut four_bytes = [0; 4];

        device.read_bytes(access, len, &mut four_bytes[..len])?;

        if is_signed && (four_bytes[len - 1] & 0x80 != 0) {
            four_bytes[len..].fill(0xff);
        }

        Ok(u32::from_le_bytes(four_bytes))
    }

    pub fn write_u32(&mut self, access: Access, data: u32) -> Result<(), Exception> {
        self.write_u32_bytes(access, data, 4)
    }

    pub fn write_u32_bytes(&mut self, mut access: Access, data: u32, len: usize) -> Result<(), Exception> {
        let device = self.mapping_mut(&mut access)?;
        device.write_bytes(access, len, &data.to_le_bytes())?;
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
    fn read_byte(&self, mut access: Access) -> Result<u8, Exception> {
        let device = self.mapping(&mut access)?;
        device.read_byte(access)
    }

    fn write_byte(&mut self, mut access: Access, data: u8) -> Result<(), Exception> {
        let device = self.mapping_mut(&mut access)?;
        device.write_byte(access, data)
    }

    fn read_bytes(&self, mut access: Access, size: usize, des: &mut [u8]) -> Result<(), Exception> {
        let device = self.mapping(&mut access)?;
        device.read_bytes(access, size, des)
    }

    fn write_bytes(&mut self, mut access: Access, size: usize, src: &[u8]) -> Result<(), Exception> {
        let device = self.mapping_mut(&mut access)?;
        device.write_bytes(access, size, src)
    }
}
