use std::io::{self, Write};

pub fn write_text(writer: &mut impl Write, text: &[u8]) -> io::Result<()> {
    let len = u32::try_from(text.len()).map_err(|_| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            "a text longer than 4 GiB does not fit a protocol length",
        )
    })?;
    writer.write_all(&len.to_be_bytes())?;
    writer.write_all(text)
}
