use thiserror::Error;

#[derive(Error, Debug)]
pub enum RiscVError {
    #[error("Register: Not exist register: {0}")]
    InvalidRegister(usize),

    #[error("Memeory: Out of bound")]
    OutOfBoundMemory,
}