use std::io::{self, Write};

pub fn write_security_types(writer: &mut impl Write, security_types: &[u8]) -> io::Result<()> {
    let count = u8::try_from(security_types.len()).map_err(|_| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            "more than 255 security types do not fit one list",
        )
    })?;
    writer.write_all(&[count])?;
    writer.write_all(security_types)
}
