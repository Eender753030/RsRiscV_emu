#[cfg(feature = "zicsr")]
pub mod csr;
pub mod register;

#[cfg(not(feature = "zicsr"))]
const MID_TITLE: &str = "Registers";
#[cfg(feature = "zicsr")]
const MID_TITLE: &str = "Reg / Csr (Press C)";
