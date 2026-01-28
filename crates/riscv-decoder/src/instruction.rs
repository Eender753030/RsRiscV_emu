//! Definition of enum corresponding to opcode
mod rv32i;
#[cfg(feature = "m")]
mod m;
#[cfg(feature = "zicsr")]
mod zicsr;
#[cfg(feature = "zifencei")]
mod zifencei;
#[cfg(feature = "zicsr")]
mod privileged;

pub use rv32i::Rv32iOp;
#[cfg(feature = "m")]
pub use m::MOp;
#[cfg(feature = "zicsr")]
pub use zicsr::ZicsrOp;
#[cfg(feature = "zifencei")]
pub use zifencei::ZifenceiOp;
#[cfg(feature = "zicsr")]
pub use privileged::PrivilegeOp;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InstructionData {
    pub rd: u8,
    pub rs1: u8,
    pub rs2: u8,
    pub imm: i32,
}

/// Definition of enum corresponding to opcode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    Base(Rv32iOp, InstructionData),
    #[cfg(feature = "zicsr")]
    Privileged(PrivilegeOp, InstructionData),
    #[cfg(feature = "m")]
    M(MOp, InstructionData),
    #[cfg(feature = "zicsr")]
    Zicsr(ZicsrOp, InstructionData),
    #[cfg(feature = "zifencei")]
    Zifencei(ZifenceiOp, InstructionData), 
}
