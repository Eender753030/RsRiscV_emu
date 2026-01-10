//! Definition of the types of Risc-V ISA

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Itype {
    pub rd: u8,
    pub rs1: u8,
    pub funct3: u8,
    pub imm: i32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rtype {
    pub rd: u8,
    pub rs1: u8,
    pub rs2: u8,
    pub funct3: u8,
    pub funct7: u8,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Stype {
    pub rs1: u8,
    pub rs2: u8,
    pub funct3: u8,
    pub imm: i32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Btype {
    pub rs1: u8,
    pub rs2: u8,
    pub funct3: u8,
    pub imm: i32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Utype {
    pub rd: u8,
    pub imm: u32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Jtype {
    pub rd: u8,
    pub imm: i32,
}