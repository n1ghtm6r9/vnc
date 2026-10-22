use std::io::{self, Write};

use crate::CLIENT_SET_ENCODINGS;

pub fn write_set_encodings(writer: &mut impl Write, encodings: &[i32]) -> io::Result<()> {
    let count = u16::try_from(encodings.len()).map_err(|_| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            "more than 65535 encodings do not fit one message",
        )
    })?;
    let mut bytes = Vec::with_capacity(4 + encodings.len() * 4);
    bytes.extend_from_slice(&[CLIENT_SET_ENCODINGS, 0]);
    bytes.extend_from_slice(&count.to_be_bytes());
    for encoding in encodings {
        bytes.extend_from_slice(&encoding.to_be_bytes());
    }
    writer.write_all(&bytes)
}
