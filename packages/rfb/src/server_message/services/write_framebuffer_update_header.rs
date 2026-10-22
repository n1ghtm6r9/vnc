use std::io::{self, Write};

use crate::SERVER_FRAMEBUFFER_UPDATE;

pub fn write_framebuffer_update_header(writer: &mut impl Write, rect_count: u16) -> io::Result<()> {
    let count = rect_count.to_be_bytes();
    writer.write_all(&[SERVER_FRAMEBUFFER_UPDATE, 0, count[0], count[1]])
}
