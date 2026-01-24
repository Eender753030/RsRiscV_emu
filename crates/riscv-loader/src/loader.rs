//! For handle file or argument from outside.

mod binary;
mod elf;

use std::path::Path;

use crate::error::LoadError;
use crate::load_info::LoadInfo;

use elf::load_elf;

pub fn load<P: AsRef<Path>>(filepath: &P) -> Result<LoadInfo, LoadError> { 
    match load_elf(filepath) {
        Ok(info) => Ok(info),
        Err(e) => {
            match e {
                LoadError::NotElfFile(raw_binary) => {
                    Ok(LoadInfo::new(0, raw_binary, 0))
                }
                _ => Err(e)
            }
        }
    }
}
