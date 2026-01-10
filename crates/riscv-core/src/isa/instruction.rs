//! Definition of enum corresponding to opcode

use super::types::*;

/// Definition of enum corresponding to opcode
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Instruction {
    Itype(Itype),
    ItypeLoad(Itype),
    ItypeJump(Itype),
    ItypeSys(Itype),
    Rtype(Rtype),
    Stype(Stype),
    Btype(Btype),
    UtypeAUIPC(Utype),
    UtypeLUI(Utype),
    Jtype(Jtype),
}
