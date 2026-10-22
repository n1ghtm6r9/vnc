use std::io::{self, Write};

use super::ServerInit;
use crate::write_text;

impl ServerInit {
    pub fn write(&self, writer: &mut impl Write) -> io::Result<()> {
        writer.write_all(&self.width.to_be_bytes())?;
        writer.write_all(&self.height.to_be_bytes())?;
        self.pixel_format.write(writer)?;
        write_text(writer, self.name.as_bytes())
    }
}
