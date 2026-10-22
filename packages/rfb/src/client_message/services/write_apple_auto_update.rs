use std::io::{self, Write};

use crate::{CLIENT_APPLE_AUTO_UPDATE, Rect};

pub fn write_apple_auto_update(writer: &mut impl Write, rect: Rect) -> io::Result<()> {
    let mut bytes = [0u8; 16];
    bytes[0] = CLIENT_APPLE_AUTO_UPDATE;
    bytes[3] = 1;
    bytes[8..10].copy_from_slice(&rect.x.to_be_bytes());
    bytes[10..12].copy_from_slice(&rect.y.to_be_bytes());
    bytes[12..14].copy_from_slice(&rect.width.to_be_bytes());
    bytes[14..16].copy_from_slice(&rect.height.to_be_bytes());
    writer.write_all(&bytes)
}
