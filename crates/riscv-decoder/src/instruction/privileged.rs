#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrivilegeOp {
    Mret, Sret,
}

impl PrivilegeOp {
    pub(crate) fn decode(raw: u32) -> Option<PrivilegeOp> {
        match raw {
            0x10200073 => Some(PrivilegeOp::Sret),
            0x30200073 => Some(PrivilegeOp::Mret),
            _ => None
        }
    }
}

impl std::fmt::Display for PrivilegeOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.pad( 
            match self {
                PrivilegeOp::Mret => "mret",
                PrivilegeOp::Sret => "sret",
            }
        )
    }
}