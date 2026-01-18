use crate::device::bus::{Bus, SystemBus};
use crate::error::RiscVError;
use crate::isa::opcode::OpCode;

pub fn load(bus: &mut SystemBus, src: u32, offset: i32, funct3: u8) -> Result<u32, RiscVError> {
    let addr = src.wrapping_add_signed(offset);
    let bytes_amount = match funct3 {
        0x0 | 0x4 => 1, // LB | LBU
        0x1 | 0x5 => 2, // LH | LHU
        0x2 => 4,       // LW
        not_exist_funct => {
            return Err(RiscVError::NotImplementedFunc(
                OpCode::ITYPE_LOAD,
                not_exist_funct,
            ));
        },
    };
    let is_signed = match funct3 {
        0x0 | 0x1 => true,        // LB | LH | LW
        0x4 | 0x5 | 0x2 => false, // LBU | LHU
        not_exist_funct => {
            return Err(RiscVError::NotImplementedFunc(
                OpCode::ITYPE_LOAD,
                not_exist_funct,
            ));
        },
    };
    bus.ram_read_u32_byte(addr, bytes_amount, is_signed)
}

pub fn store(
    bus: &mut SystemBus,
    des: u32,
    src: u32,
    offset: i32,
    funct3: u8,
) -> Result<(), RiscVError> {
    let addr = des.wrapping_add_signed(offset);

    let bytes_amount = match funct3 {
        0x0 => 1, // SB
        0x1 => 2, // SH
        0x2 => 4, // SW
        not_exist_funct => {
            return Err(RiscVError::NotImplementedFunc(
                OpCode::STYPE,
                not_exist_funct,
            ));
        },
    };
    bus.ram.write_bytes(addr, bytes_amount, &src.to_le_bytes())
}
