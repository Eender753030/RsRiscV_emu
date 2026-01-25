//! For handle file or argument from outside.

mod binary;
mod elf;

use std::path::Path;

use crate::error::LoadError;
use crate::load_info::LoadInfo;

use elf::load_elf;

pub fn load<P: AsRef<Path>>(filepath: &P) -> Result<LoadInfo, LoadError> { 
    load_elf(filepath).or_else(|e| match e {
        LoadError::NotElfFile(raw_binary) => Ok(LoadInfo::from_raw_binary(raw_binary)),
        _   => Err(e),
    })
}
