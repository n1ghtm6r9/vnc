use std::io::{self, Write};

use crate::CLIENT_APPLE_SET_MODE;

pub fn write_apple_set_mode(writer: &mut impl Write, mode: u8) -> io::Result<()> {
    writer.write_all(&[CLIENT_APPLE_SET_MODE, 0, 0, mode])
}
