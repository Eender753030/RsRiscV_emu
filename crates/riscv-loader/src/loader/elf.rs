use std::collections::HashMap;
use std::path::Path;

use elf::{ElfBytes, abi};
use elf::endian::LittleEndian;
use elf::file::Class;

use crate::error::LoadError;
use crate::load_info::LoadInfo;

use super::binary::read_binary;

pub fn load_elf<P: AsRef<Path>>(filepath: &P) -> Result<LoadInfo, LoadError> {
    let file_data = read_binary(filepath)?;
    
    let res = ElfBytes::<LittleEndian>::minimal_parse(file_data.as_slice());

    let file = match res {
        Ok(file) => file,
        Err(e) => return Err(match e {
            elf::ParseError::BadMagic(_) => LoadError::NotElfFile(file_data),
            e =>  LoadError::ParseElfFailed(e.to_string()),
        })
    };
    
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

        info.header_sections = get_elf_header_sections(&file)
            .map_err(|e| println!("Warning: {}", e))
            .ok();

        info.symbols = get_elf_symtab(&file)
            .map_err(|e| println!("Warning: {}", e))
            .ok();
        
        Ok(info)
    } else {
        Err(LoadError::ReadProgramHeadersFailed)
    }
}

fn get_elf_header_sections(elf_file : &ElfBytes<LittleEndian>) -> Result<Vec<(String, u32)>, LoadError> {
    let (shdrs_opt, strtab_opt) = elf_file.section_headers_with_strtab()
        .map_err(|e| LoadError::GetElfSectionHeaderFailed(e.to_string()))?;

    let mut sections = Vec::new();

    if let Some(shdrs) = shdrs_opt && let Some(strtab) = strtab_opt {
        shdrs.iter()
            .filter(|shdr| shdr.sh_type == abi::SHT_PROGBITS)
            .try_for_each(|shdr| {
                let name = strtab.get(shdr.sh_name as usize)
                    .map_err(|e| LoadError::GetElfSectionHeaderNameFailed(e.to_string()))?;
                sections.push((name.to_string(), shdr.sh_addr as u32));
                Ok(())
            })?;
        Ok(sections)
    } else {
        Err(LoadError::GetElfSectionHeaderFailed("No header section".to_string()))
    }
}

fn get_elf_symtab(elf_file : &ElfBytes<LittleEndian>) -> Result<HashMap<u32, String>, LoadError> {
    let opt = elf_file.symbol_table()
        .map_err(|e| LoadError::GetElfSectionHeaderFailed(e.to_string()))?;

    let mut syms = HashMap::new();

    if let Some((symtab, strtab)) = opt {
        symtab.iter()
            .filter(|sym| 
                matches!(sym.st_symtype(), abi::STT_FUNC | abi::STT_OBJECT | abi::STT_NOTYPE) 
            )
            .try_for_each(|sym| {
                let name = strtab.get(sym.st_name as usize)
                    .map_err(|e| LoadError::GetElfSectionHeaderNameFailed(e.to_string()))?;
                let val = sym.st_value as u32;
                if !name.is_empty() && !name.starts_with("$") 
                    && val != 0 && sym.st_shndx != abi::SHN_UNDEF {
                    syms.insert(val, name.to_string());
                }
                Ok(())
            })?;
        Ok(syms)
    } else {
        Err(LoadError::GetElfSymbolFailed("No symbol".to_string()))
    }
}