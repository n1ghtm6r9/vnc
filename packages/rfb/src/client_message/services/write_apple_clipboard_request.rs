use std::io::{self, Write};

use crate::CLIENT_APPLE_CLIPBOARD_REQUEST;

pub fn write_apple_clipboard_request(writer: &mut impl Write, session_id: u32) -> io::Result<()> {
    let mut bytes = [0u8; 8];
    bytes[0] = CLIENT_APPLE_CLIPBOARD_REQUEST;
    bytes[4..].copy_from_slice(&session_id.to_be_bytes());
    writer.write_all(&bytes)
}
