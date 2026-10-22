use std::io::{self, Read};

use super::PixelFormat;

impl PixelFormat {
    pub fn read(reader: &mut impl Read) -> io::Result<PixelFormat> {
        let mut bytes = [0u8; 16];
        reader.read_exact(&mut bytes)?;
        Ok(PixelFormat {
            bits_per_pixel: bytes[0],
            depth: bytes[1],
            big_endian: bytes[2] != 0,
            true_colour: bytes[3] != 0,
            red_max: u16::from_be_bytes([bytes[4], bytes[5]]),
            green_max: u16::from_be_bytes([bytes[6], bytes[7]]),
            blue_max: u16::from_be_bytes([bytes[8], bytes[9]]),
            red_shift: bytes[10],
            green_shift: bytes[11],
            blue_shift: bytes[12],
        })
    }
}
