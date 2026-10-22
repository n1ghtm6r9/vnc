use std::io::{self, Write};

use crate::CLIENT_APPLE_ENCRYPTED_EVENT;

pub fn write_apple_encrypted_event(writer: &mut impl Write, block: &[u8; 16]) -> io::Result<()> {
    let mut bytes = [0u8; 18];
    bytes[0] = CLIENT_APPLE_ENCRYPTED_EVENT;
    bytes[2..].copy_from_slice(block);
    writer.write_all(&bytes)
}
