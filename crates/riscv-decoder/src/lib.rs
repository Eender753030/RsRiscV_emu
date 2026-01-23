pub mod decoder;
pub mod instruction;
mod opcode;
mod bits_op;
mod csr_addr;
mod error;
pub mod prelude;

pub use error::DecodeError;
