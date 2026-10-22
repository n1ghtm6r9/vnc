use std::io::{self, Write};

use crate::{CLIENT_APPLE_CLIPBOARD, apple_pasteboard, deflate_pasteboard};

pub fn write_apple_clipboard(writer: &mut impl Write, session_id: u32, text: &str) -> io::Result<()> {
    let pasteboard = apple_pasteboard(text);
    let compressed = deflate_pasteboard(&pasteboard)?;
    let length = |bytes: &[u8]| {
        u32::try_from(bytes.len()).map_err(|_| {
            io::Error::new(io::ErrorKind::InvalidInput, "the clipboard text is longer than 4 GiB")
        })
    };
    let mut bytes = Vec::with_capacity(16 + compressed.len());
    bytes.extend_from_slice(&[CLIENT_APPLE_CLIPBOARD, 0, 0, 0]);
    bytes.extend_from_slice(&session_id.to_be_bytes());
    bytes.extend_from_slice(&length(&pasteboard)?.to_be_bytes());
    bytes.extend_from_slice(&length(&compressed)?.to_be_bytes());
    bytes.extend_from_slice(&compressed);
    writer.write_all(&bytes)
}
