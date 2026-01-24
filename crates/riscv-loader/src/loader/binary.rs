use std::fs;
use std::io::{self, Read};
use std::path::Path;

use crate::error::LoadError;

/// Access binary file of `filename` and return its content by `Vec<u8>`.
/// Risc-V is Little Endian.
/// ## Example
/// ```bin
/// ```
/// ```rust,no_run
/// # use risc_v_emulator::riscv::loader;
/// let filename: &str = "binary_file";
/// 
/// let binary_content = loader::read_binary(filename);
/// ```
pub fn read_binary<P: AsRef<Path>>(filepath: &P) -> Result<Vec<u8>, LoadError>{
    let file = fs::File::open(filepath).map_err(|e| 
        LoadError::OpenFileFailed(e.to_string())
    )?;
    
    let mut reader = io::BufReader::new(file);

    let mut content = Vec::new();

    reader.read_to_end(&mut content).map_err(|e|
        LoadError::ReadRawBinaryFailed(e.to_string())
    )?;

    Ok(content)
}