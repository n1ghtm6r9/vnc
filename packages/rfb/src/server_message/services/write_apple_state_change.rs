use std::io::{self, Write};

use crate::SERVER_APPLE_STATE_CHANGE;

pub fn write_apple_state_change(writer: &mut impl Write, status: u16) -> io::Result<()> {
    let [high, low] = status.to_be_bytes();
    writer.write_all(&[SERVER_APPLE_STATE_CHANGE, 0, 0, 4, 0, 0, high, low])
}
