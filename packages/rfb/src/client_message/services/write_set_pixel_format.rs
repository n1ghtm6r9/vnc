use std::io::{self, Write};

use crate::{CLIENT_SET_PIXEL_FORMAT, PixelFormat};

pub fn write_set_pixel_format(writer: &mut impl Write, format: &PixelFormat) -> io::Result<()> {
    writer.write_all(&[CLIENT_SET_PIXEL_FORMAT, 0, 0, 0])?;
    format.write(writer)
}
