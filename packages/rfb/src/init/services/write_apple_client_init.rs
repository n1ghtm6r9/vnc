use std::io::{self, Write};

use crate::APPLE_CLIENT_INIT;

pub fn write_apple_client_init(writer: &mut impl Write) -> io::Result<()> {
    writer.write_all(&[APPLE_CLIENT_INIT])
}
