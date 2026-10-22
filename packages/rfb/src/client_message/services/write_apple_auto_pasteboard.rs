use std::io::{self, Write};

use crate::CLIENT_APPLE_AUTO_PASTEBOARD;

pub fn write_apple_auto_pasteboard(writer: &mut impl Write, enabled: bool) -> io::Result<()> {
    writer.write_all(&[CLIENT_APPLE_AUTO_PASTEBOARD, 0, 0, u8::from(enabled), 0, 0, 0, 0])
}
