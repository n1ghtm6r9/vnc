use std::io::{self, Read};

pub fn read_u16(reader: &mut impl Read) -> io::Result<u16> {
    let mut buffer = [0u8; 2];
    reader.read_exact(&mut buffer)?;
    Ok(u16::from_be_bytes(buffer))
}
