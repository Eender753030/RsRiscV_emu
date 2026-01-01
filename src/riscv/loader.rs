//! For handle file or argument from outside.

use crate::utils::exception::CliError;

use std::{io::{self, Read}, fs, env};

/// Load CLI argument from `env::args().skip(1)`. Only accept one binary file for now.
pub fn load_arg() -> Result<std::iter::Skip<env::Args>, CliError>{
    let args = env::args().skip(1);

    if args.len() == 0 {
        Err(CliError::NoInputBinary)
    } else if args.len() > 1 {
        Err(CliError::TooManyArgument)
    } else {
        Ok(args)
    }
}

/// Access binary file of `filename` and return its content by `Vec<u8>`.
/// Risc-V is Little Endian.
pub fn read_binary(filename: &str) -> io::Result<Vec<u8>>{
    let file = fs::File::open(filename)?;
    let mut reader = io::BufReader::new(file);

    let mut content = Vec::new();

    reader.read_to_end(&mut content)?;

    Ok(content)
}