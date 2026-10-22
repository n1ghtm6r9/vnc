use std::io::{self, BufWriter, Write};

use rfb::{
    PixelFormat, Rect, write_framebuffer_update_request, write_set_encodings,
    write_set_pixel_format,
};

pub(crate) fn send_setup(
    stream: &mut impl Write,
    encodings: &[i32],
    screen: Rect,
) -> io::Result<()> {
    let mut output = BufWriter::new(stream);
    write_set_pixel_format(&mut output, &PixelFormat::rgba32())?;
    write_set_encodings(&mut output, encodings)?;
    write_framebuffer_update_request(&mut output, false, screen)?;
    output.flush()
}
