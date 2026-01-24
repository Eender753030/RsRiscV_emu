#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Exception {
    InstructionAddressMisaligned,
    InstructionAccessFault(u32),
    IllegalInstruction(u32),
    Breakpoint,
    LoadAddressMisaligned,
    LoadAccessFault(u32),
    StoreAddressMisaligned,
    StoreAccessFault(u32),
    EnvironmentCallFromUMode,
    EnvironmentCallFromSMode,
    EnvironmentCallFromMMode,
    InstructionPageFault(u32),
    LoadPageFault(u32),
    StoreOrAmoPageFault(u32),
}

impl From<Exception> for u32 {
    fn from(value: Exception) -> Self {
        match value {
            Exception::InstructionAddressMisaligned => 0,
            Exception::InstructionAccessFault(_) => 1,
            Exception::IllegalInstruction(_) => 2,
            Exception::Breakpoint => 3,
            Exception::LoadAddressMisaligned => 4,
            Exception::LoadAccessFault(_) => 5,
            Exception::StoreAddressMisaligned => 6,
            Exception::StoreAccessFault(_) => 7,
            Exception::EnvironmentCallFromUMode => 8,
            Exception::EnvironmentCallFromSMode => 9,
            Exception::EnvironmentCallFromMMode => 11,
            Exception::InstructionPageFault(_) => 12,
            Exception::LoadPageFault(_) => 13,
            Exception::StoreOrAmoPageFault(_) => 15,
        }
    }
}
