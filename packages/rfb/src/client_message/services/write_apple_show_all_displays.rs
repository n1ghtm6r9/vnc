use std::io::{self, Write};

use crate::CLIENT_APPLE_SET_DISPLAY;

pub fn write_apple_show_all_displays(writer: &mut impl Write) -> io::Result<()> {
    writer.write_all(&[CLIENT_APPLE_SET_DISPLAY, 1, 0, 0, 0, 0, 0, 0])
}
