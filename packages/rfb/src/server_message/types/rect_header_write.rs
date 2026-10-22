use std::io::{self, Write};

use super::RectHeader;

impl RectHeader {
    pub fn write(&self, writer: &mut impl Write) -> io::Result<()> {
        let mut bytes = [0u8; 12];
        bytes[0..2].copy_from_slice(&self.rect.x.to_be_bytes());
        bytes[2..4].copy_from_slice(&self.rect.y.to_be_bytes());
        bytes[4..6].copy_from_slice(&self.rect.width.to_be_bytes());
        bytes[6..8].copy_from_slice(&self.rect.height.to_be_bytes());
        bytes[8..12].copy_from_slice(&self.encoding.to_be_bytes());
        writer.write_all(&bytes)
    }
}
