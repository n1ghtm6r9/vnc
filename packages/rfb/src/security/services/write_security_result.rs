use std::io::{self, Write};

use crate::write_text;

pub fn write_security_result(writer: &mut impl Write, failure_reason: Option<&str>) -> io::Result<()> {
    match failure_reason {
        None => writer.write_all(&0u32.to_be_bytes()),
        Some(reason) => {
            writer.write_all(&1u32.to_be_bytes())?;
            write_text(writer, reason.as_bytes())
        }
    }
}
