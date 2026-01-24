use crate::core::{Access, AccessType, Mmu, PrivilegeMode};
use crate::device::bus::SystemBus;
use crate::exception::Exception;

pub struct Lsu;

impl Lsu {
    pub fn load(bus: &mut SystemBus, src: u32, offset: i32, num: usize, mode: PrivilegeMode, ppn_opt: Option<u32>) -> Result<u32, Exception> {
        let addr = src.wrapping_add_signed(offset);
        let va_access = Access::new(addr, AccessType::Load);
        let pa_access = Mmu::translate(va_access, mode, ppn_opt, bus)?;
        match bus.read_u32_bytes(pa_access, num, false) {
            Ok(data) => Ok(data),
            Err(e) => {
                match e {
                    Exception::LoadAccessFault(_) => Err(Exception::LoadAccessFault(addr)),
                    Exception::StoreAccessFault(_) => Err(Exception::StoreAccessFault(addr)),
                    _ => Err(e),
                }
            }
        }
    }

    pub fn load_signed(bus: &mut SystemBus, src: u32, offset: i32, num: usize, mode: PrivilegeMode, ppn_opt: Option<u32>) -> Result<u32, Exception> {
        let addr = src.wrapping_add_signed(offset);
        let va_access = Access::new(addr, AccessType::Load);
        let pa_access = Mmu::translate(va_access, mode, ppn_opt, bus)?;   
        match bus.read_u32_bytes(pa_access, num, true) {
            Ok(data) => Ok(data),
            Err(e) => {
                match e {
                    Exception::LoadAccessFault(_) => Err(Exception::LoadAccessFault(addr)),
                    Exception::StoreAccessFault(_) => Err(Exception::StoreAccessFault(addr)),
                    _ => Err(e),
                }
            }
        }
    }

    pub fn store(bus: &mut SystemBus, des: u32, src: u32, offset: i32, num: usize, mode: PrivilegeMode, ppn_opt: Option<u32>) -> Result<(), Exception> {
        let addr = des.wrapping_add_signed(offset);
        let va_access = Access::new(addr, AccessType::Store);
        let pa_access = Mmu::translate(va_access, mode, ppn_opt, bus)?;   
        match bus.write_u32_bytes(pa_access, src, num) {
            Ok(_) => Ok(()),
            Err(e) => {
                match e {
                    Exception::LoadAccessFault(_) => Err(Exception::LoadAccessFault(addr)),
                    Exception::StoreAccessFault(_) => Err(Exception::StoreAccessFault(addr)),
                    _ => Err(e),
                }
            }
        }
    }
}
