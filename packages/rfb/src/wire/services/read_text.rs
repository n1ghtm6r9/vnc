use std::io::{self, Read};

use super::read_text_bytes;

pub fn read_text(reader: &mut impl Read) -> io::Result<String> {
    let bytes = read_text_bytes(reader)?;
    Ok(String::from_utf8_lossy(&bytes).into_owned())
}
