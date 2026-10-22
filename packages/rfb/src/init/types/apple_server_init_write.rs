use std::io::{self, Write};

use super::AppleServerInit;
use crate::write_text;

impl AppleServerInit {
    pub fn write(&self, writer: &mut impl Write) -> io::Result<()> {
        writer.write_all(&self.init.width.to_be_bytes())?;
        writer.write_all(&self.init.height.to_be_bytes())?;
        self.init.pixel_format.write(writer)?;
        let mut field = vec![0, 0];
        field.extend_from_slice(&self.flags.to_be_bytes());
        field.extend_from_slice(&self.capabilities);
        field.push(0);
        field.extend_from_slice(self.init.name.as_bytes());
        write_text(writer, &field)
    }
}
