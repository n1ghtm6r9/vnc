use std::io::{self, Write};

use super::PixelFormat;

impl PixelFormat {
    pub fn write(&self, writer: &mut impl Write) -> io::Result<()> {
        let mut bytes = [0u8; 16];
        bytes[0] = self.bits_per_pixel;
        bytes[1] = self.depth;
        bytes[2] = u8::from(self.big_endian);
        bytes[3] = u8::from(self.true_colour);
        bytes[4..6].copy_from_slice(&self.red_max.to_be_bytes());
        bytes[6..8].copy_from_slice(&self.green_max.to_be_bytes());
        bytes[8..10].copy_from_slice(&self.blue_max.to_be_bytes());
        bytes[10] = self.red_shift;
        bytes[11] = self.green_shift;
        bytes[12] = self.blue_shift;
        writer.write_all(&bytes)
    }
}
