use std::io::{self, Write};

use crate::CLIENT_APPLE_SET_INPUT_SOURCE;

pub fn write_apple_set_input_source(writer: &mut impl Write, source: &str) -> io::Result<()> {
    let name = source.as_bytes();
    let size = u16::try_from(name.len() + 4).map_err(|_| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            "an input source name longer than 65531 bytes does not fit one message",
        )
    })?;
    let mut bytes = Vec::with_capacity(8 + name.len());
    bytes.extend_from_slice(&[CLIENT_APPLE_SET_INPUT_SOURCE, 0]);
    bytes.extend_from_slice(&size.to_be_bytes());
    bytes.extend_from_slice(&1u16.to_be_bytes());
    bytes.extend_from_slice(&(size - 4).to_be_bytes());
    bytes.extend_from_slice(name);
    writer.write_all(&bytes)
}
