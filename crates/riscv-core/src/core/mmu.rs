mod sv32;

pub mod access;

use crate::Exception;
use crate::core::privilege::PrivilegeMode;
use crate::device::bus::SystemBus;

use sv32::{Sv32Pte, Sv32Vpn};
use access::{Access, AccessType, Physical, Virtual};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Mmu;

impl Mmu {
    pub fn translate(access: Access<Virtual>, mode: PrivilegeMode, ppn_opt: Option<u32>, bus: &mut SystemBus) -> Result<Access<Physical>, Exception> {
        let v_addr = access.addr; 
        Ok(match mode {
            PrivilegeMode::Machine => access.bypass(),
            PrivilegeMode::Supervisor | PrivilegeMode::User => {
                if let Some(ppn) = ppn_opt {
                    let vpn = Sv32Vpn::from(v_addr);
                    let (pte1, pte1_addr, is_leaf) = Self::pte_walk(
                        vpn.vpn_1() as u32, ppn, &access, bus)?;

                    let pte0_opt = if is_leaf {
                        None
                    } else {
                        let (pte0, pte0_addr, is_leaf) = Self::pte_walk(
                            vpn.vpn_0() as u32, pte1.ppn(), &access, bus)?;
                        if !is_leaf {
                            return Err(access.to_page_exception());
                        }
                        Some((pte0, pte0_addr))
                    };

                    let (mut leaf_pte, leaf_pte_addr) = if let Some((pte0, addr)) = pte0_opt {
                        (pte0, addr)
                    } else {
                        (pte1, pte1_addr)
                    };

                    Self::access_check(&leaf_pte, access, mode)?;

                    let leaf_pte_access = Access::new(leaf_pte_addr, access.kind);
                    if leaf_pte.is_access_zero_and_set() {
                        bus.write_u32(leaf_pte_access, leaf_pte.into())?;
                    }
                    if access.kind == AccessType::Store && leaf_pte.is_dirty_zero_and_set() {
                        bus.write_u32(leaf_pte_access, leaf_pte.into())?;
                    }
                    
                    let p_addr = if pte0_opt.is_some() {
                        (leaf_pte.ppn() << 12) | vpn.offset() as u32
                    } else {
                        let ppn_0 = leaf_pte.ppn() & 0x3ff;
                        if ppn_0 != 0 {
                            return Err(access.to_page_exception());
                        }
                        let ppn_1 = leaf_pte.ppn() & 0x3ffc00;
                        ppn_1 << 12 | (vpn.vpn_0() as u32) << 12 | vpn.offset() as u32
                    };
                    access.into_physical(p_addr)
                } else {
                    access.bypass()
                }
            }
        })
    }

    fn pte_walk(vpn: u32, ppn: u32, access: &Access, bus: &mut SystemBus) -> Result<(Sv32Pte, u32, bool), Exception> {
        let pte_addr = (ppn << 12) + vpn * 4;

        let pte_access = Access::new(pte_addr, AccessType::Load);

        let pte = Sv32Pte::from(bus.read_u32(pte_access)?);

        Ok(if !pte.is_valid() || (!pte.can_read() && pte.can_write()) {
            return  Err(access.to_page_exception())
        } else if pte.is_leaf() {
            (pte, pte_addr, true)
        } else {
            (pte, pte_addr, false)
        })
    }

    fn access_check(pte: &Sv32Pte, access: Access, mode: PrivilegeMode) -> Result<(), Exception> {
        let can_access = match access.kind {
            AccessType::Load  => !pte.can_read(),
            AccessType::Store => !pte.can_write(),
            AccessType::Fetch => !pte.can_execute(),
        };
        
        let can_mode = 
            mode == PrivilegeMode::User       && !pte.can_user() ||
            mode == PrivilegeMode::Supervisor && pte.can_user();
         
        if can_access && can_mode {
            Ok(())
        } else {
            Err(access.to_page_exception())
        }
    }
}
