use std::io::{self, Write};

use crate::{CLIENT_FRAMEBUFFER_UPDATE_REQUEST, Rect};

pub fn write_framebuffer_update_request(
    writer: &mut impl Write,
    incremental: bool,
    rect: Rect,
) -> io::Result<()> {
    let mut bytes = [0u8; 10];
    bytes[0] = CLIENT_FRAMEBUFFER_UPDATE_REQUEST;
    bytes[1] = u8::from(incremental);
    bytes[2..4].copy_from_slice(&rect.x.to_be_bytes());
    bytes[4..6].copy_from_slice(&rect.y.to_be_bytes());
    bytes[6..8].copy_from_slice(&rect.width.to_be_bytes());
    bytes[8..10].copy_from_slice(&rect.height.to_be_bytes());
    writer.write_all(&bytes)
}
