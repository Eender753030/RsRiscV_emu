//! For handle file or argument from outside.

use elf::abi;
use elf::endian::LittleEndian;
use elf::ElfBytes;
use elf::file::Class;

use crate::error::LoadError;
use crate::load_info::LoadInfo;

use std::io::{self, Read};
use std::fs;
use std::path::Path;

pub fn load<P: AsRef<Path>>(filepath: &P) -> Result<LoadInfo, LoadError> { 
    match load_elf(filepath) {
        Ok(info) => Ok(info),
        Err(e) => {
            match e {
                LoadError::NotElfFile => {
                    let code = read_binary(filepath)?;
                    Ok(LoadInfo::new(0, code, 0))
                }
                _ => Err(e)
            }
        }
    }
}

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
fn read_binary<P: AsRef<Path>>(filepath: &P) -> Result<Vec<u8>, LoadError>{
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

fn load_elf<P: AsRef<Path>>(filepath: &P) -> Result<LoadInfo, LoadError> {
    let file_data = read_binary(filepath)?;
    
    let file = ElfBytes::<LittleEndian>::minimal_parse(file_data.as_slice()).map_err(|e|
        match e {
            elf::ParseError::BadMagic(_) => LoadError::NotElfFile,
            e =>  LoadError::ReadElfFailed(e.to_string()),
        }
    )?;
    
    if file.ehdr.e_machine != abi::EM_RISCV {
        return Err(LoadError::NotRiscVArc(file.ehdr.e_machine));
    } 
    
    if file.ehdr.class != Class::ELF32 {
        return Err(LoadError::NotSupportClass);
    }

    if let Some(segments) = file.segments() {
        let mut info = LoadInfo::default();
        
        segments.iter()
            .filter(|seg| seg.p_type == abi::PT_LOAD)
            .for_each(|seg| {
                let addr = seg.p_vaddr as u32;
                let mem_size = seg.p_memsz as usize;
                let file_size = seg.p_filesz as usize;
                let offset = seg.p_offset as usize;
                
                let is_code = (seg.p_flags & !(abi::PF_R | abi::PF_X)) == 0;
                let is_data = (seg.p_flags & !(abi::PF_R | abi::PF_W)) == 0;

                if file_size > 0 {
                    let data = file_data[offset..offset + file_size].to_vec();
                    if is_code {
                        info.push_code(data, addr);
                    } else if is_data {
                        info.push_data(data, addr);
                        if mem_size > file_size {
                            let bss_size = mem_size - file_size;
                            let bss_start = addr + file_size as u32;
                            info.set_bss(bss_start, bss_size);
                        }
                    } else {
                        info.push_other(data, addr);
                    }
                }     
            });
        info.pc_entry = file.ehdr.e_entry as u32;
        Ok(info)
    } else {
        Err(LoadError::ReadProgramHeadersFailed)
    }
}
