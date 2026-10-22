use std::io::{self, Write};

use crate::CLIENT_KEY_EVENT;

pub fn write_key_event(writer: &mut impl Write, down: bool, keysym: u32) -> io::Result<()> {
    let mut bytes = [0u8; 8];
    bytes[0] = CLIENT_KEY_EVENT;
    bytes[1] = u8::from(down);
    bytes[4..8].copy_from_slice(&keysym.to_be_bytes());
    writer.write_all(&bytes)
}
