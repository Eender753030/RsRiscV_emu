pub enum Action {
    ADDI,
}

pub enum TypeKind {
    IType
}
pub enum Types {
    IType {imm: u32, rs1: usize, rd: usize, action: Action},
}

impl Types {
    pub fn parse(op_type: TypeKind, ins: u32, action: Action) -> Self {
        match op_type{
            TypeKind::IType => {
                Types::IType {imm: ((ins >> 20) & 0xfff) as u32,
                       rs1: ((ins >> 11) & 0x1f) as usize,
                       rd:  ((ins >> 7) & 0x1f) as usize,
                       action}
            }
        }
    }
}