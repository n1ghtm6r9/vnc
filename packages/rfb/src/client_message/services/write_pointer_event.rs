use std::io::{self, Write};

use crate::CLIENT_POINTER_EVENT;

pub fn write_pointer_event(writer: &mut impl Write, buttons: u8, x: u16, y: u16) -> io::Result<()> {
    let mut bytes = [0u8; 6];
    bytes[0] = CLIENT_POINTER_EVENT;
    bytes[1] = buttons;
    bytes[2..4].copy_from_slice(&x.to_be_bytes());
    bytes[4..6].copy_from_slice(&y.to_be_bytes());
    writer.write_all(&bytes)
}
