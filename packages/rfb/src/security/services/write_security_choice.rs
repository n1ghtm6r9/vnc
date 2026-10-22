use std::io::{self, Write};

pub fn write_security_choice(writer: &mut impl Write, security_type: u8) -> io::Result<()> {
    writer.write_all(&[security_type])
}
