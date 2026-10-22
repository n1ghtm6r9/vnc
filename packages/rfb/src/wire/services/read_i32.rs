use std::io::{self, Read};

pub fn read_i32(reader: &mut impl Read) -> io::Result<i32> {
    let mut buffer = [0u8; 4];
    reader.read_exact(&mut buffer)?;
    Ok(i32::from_be_bytes(buffer))
}
