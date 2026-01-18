//! The const of magic number for decoding

/// Define const of every instruction corresponding to opcode
pub struct OpCode;

impl OpCode {
    pub const ITYPE: u8 = 0x13;
    pub const ITYPE_LOAD: u8 = 0x03;
    pub const ITYPE_JUMP: u8 = 0x67;
    pub const ITYPE_SYS: u8 = 0x73;
    pub const RTYPE: u8 = 0x33;
    pub const STYPE: u8 = 0x23;
    pub const BTYPE: u8 = 0x63;
    pub const JTYPE: u8 = 0x6f;
    pub const UTYPE_AUIPC: u8 = 0x17;
    pub const UTYPE_LUI: u8 = 0x37;
}

/// Define bits position for decoding  
pub struct Bits;

impl Bits {
    pub const _31: u32 = 0b1000_0000_0000_0000_0000_0000_0000_0000;

    pub const _31_TO_25: u32 = 0b1111_1110_0000_0000_0000_0000_0000_0000;

    pub const _31_TO_20: u32 = 0b1111_1111_1111_0000_0000_0000_0000_0000;

    pub const _31_TO_12: u32 = 0b1111_1111_1111_1111_1111_0000_0000_0000;

    pub const _30_TO_25: u32 = 0b0111_1110_0000_0000_0000_0000_0000_0000;

    pub const _30_TO_21: u32 = 0b0111_1111_1110_0000_0000_0000_0000_0000;

    pub const _24_TO_20: u32 = 0b0000_0001_1111_0000_0000_0000_0000_0000;

    pub const _20: u32 = 0b0000_0000_0001_0000_0000_0000_0000_0000;

    pub const _19_TO_15: u32 = 0b0000_0000_0000_1111_1000_0000_0000_0000;

    pub const _19_TO_12: u32 = 0b0000_0000_0000_1111_1111_0000_0000_0000;

    pub const _14_TO_12: u32 = 0b0000_0000_0000_0000_0111_0000_0000_0000;

    pub const _11_TO_8: u32 = 0b0000_0000_0000_0000_0000_1111_0000_0000;

    pub const _11_TO_7: u32 = 0b0000_0000_0000_0000_0000_1111_1000_0000;

    pub const _7: u32 = 0b0000_0000_0000_0000_0000_0000_1000_0000;

    pub const _6_TO_0: u32 = 0b0000_0000_0000_0000_0000_0000_0111_1111;
}
