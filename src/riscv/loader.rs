use std::fs;
use std::io::{self, Read};

pub fn read_binary(filename: &str) -> io::Result<Vec<u8>>{
    let file = fs::File::open(filename)?;
    let mut reader = io::BufReader::new(file);

    let mut content = Vec::new();

    reader.read_to_end(&mut content)?;

    Ok(content)
}