use std::io::{self, Read};

pub fn read_u8(reader: &mut impl Read) -> io::Result<u8> {
    let mut buffer = [0u8; 1];
    reader.read_exact(&mut buffer)?;
    Ok(buffer[0])
}
