#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CsrAddr {
    Ustatus = 0x000,
    Satp = 0x180,
    Mstatus = 0x300,
    Medeleg = 0x302,
    Mideleg = 0x303,
    Mie = 0x304,
    Mtvec = 0x305,
    Mepc = 0x341,
    Mcause = 0x342,
    Pmpcfg0 = 0x3a0,
    Pmpaddr0 = 0x3b0,
    Mnscratch = 0x744,
    Mhartid = 0xf14,
}

impl std::fmt::Display for CsrAddr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.pad( 
            match self {
                CsrAddr::Ustatus => "ustatus",
                CsrAddr::Satp => "satp",
                CsrAddr::Mstatus => "mstatus",
                CsrAddr::Medeleg => "medeleg",
                CsrAddr::Mideleg => "mideleg",
                CsrAddr::Mie => "mie",
                CsrAddr::Mtvec => "mtvec",
                CsrAddr::Mepc => "mepc",
                CsrAddr::Mcause => "mcause",
                CsrAddr::Pmpcfg0 => "pmpcfg0",
                CsrAddr::Pmpaddr0 => "pmpaddr0",
                CsrAddr::Mnscratch => "mnscratch",
                CsrAddr::Mhartid => "mhartid",
            }
        )
    }
}

impl TryFrom<u32> for CsrAddr {
    type Error = u32;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0x000 => Ok(CsrAddr::Ustatus),
            0x180 => Ok(CsrAddr::Satp),
            0x300 => Ok(CsrAddr::Mstatus),
            0x302 => Ok(CsrAddr::Medeleg),
            0x303 => Ok(CsrAddr::Mideleg),
            0x304 => Ok(CsrAddr::Mie),
            0x305 => Ok(CsrAddr::Mtvec),
            0x341 => Ok(CsrAddr::Mepc),
            0x342 => Ok(CsrAddr::Mcause),
            0x3a0 => Ok(CsrAddr::Pmpcfg0),
            0x3b0 => Ok(CsrAddr::Pmpaddr0),
            0x744 => Ok(CsrAddr::Mnscratch),
            0xf14 => Ok(CsrAddr::Mhartid),  
            _ => Err(value),
        }
    }
}