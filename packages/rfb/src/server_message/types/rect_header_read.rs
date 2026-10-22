use std::io::{self, Read};

use super::RectHeader;
use crate::Rect;

impl RectHeader {
    pub fn read(reader: &mut impl Read) -> io::Result<RectHeader> {
        let mut bytes = [0u8; 12];
        reader.read_exact(&mut bytes)?;
        let rect = Rect::new(
            u16::from_be_bytes([bytes[0], bytes[1]]),
            u16::from_be_bytes([bytes[2], bytes[3]]),
            u16::from_be_bytes([bytes[4], bytes[5]]),
            u16::from_be_bytes([bytes[6], bytes[7]]),
        );
        let encoding = i32::from_be_bytes([bytes[8], bytes[9], bytes[10], bytes[11]]);
        Ok(RectHeader { rect, encoding })
    }
}
