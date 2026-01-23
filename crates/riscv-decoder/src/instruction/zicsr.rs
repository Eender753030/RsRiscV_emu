#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ZicsrOp {
    Csrrw, Csrrs, Csrrc,
    Csrrwi, Csrrsi, Csrrci,
    
}

impl ZicsrOp {
    pub(crate) fn decode(funct3: u8) -> Option<ZicsrOp> {
        match funct3 {
            0x1 => Some(ZicsrOp::Csrrw),
            0x2 => Some(ZicsrOp::Csrrs),
            0x3 => Some(ZicsrOp::Csrrc),
            0x5 => Some(ZicsrOp::Csrrwi),
            0x6 => Some(ZicsrOp::Csrrsi),
            0x7 => Some(ZicsrOp::Csrrci),
            _ => None
        }
    }
}

impl std::fmt::Display for ZicsrOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.pad( 
            match self {
                ZicsrOp::Csrrw => "csrrw", ZicsrOp::Csrrs => "csrrs", ZicsrOp::Csrrc => "csrrc",
                ZicsrOp::Csrrwi => "csrrwi", ZicsrOp::Csrrsi => "csrrsi", ZicsrOp::Csrrci => "csrrci",
            }
        )
    }
}