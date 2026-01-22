use thiserror::Error;

#[derive(Error, Debug)]
pub enum LoadError {
    #[error("Architecture: {0} is not Risc-V (243)")]
    NotRiscVArc(u16),

    #[error("Can not open file: {0}")]
    OpenFileFailed(String),

    #[error("Can not read raw binary: {0}")]
    ReadRawBinaryFailed(String),

    #[error("Only support 32-bit now")]
    NotSupportClass,

    #[error("Can not read .elf's program headers")]
    ReadProgramHeadersFailed,

    #[error("Not .elf file")]
    NotElfFile,

    #[error("Can not read .elf: {0}")]
    ReadElfFailed(String),
}
