use std::io::{self, Write};

use crate::SERVER_BELL;

pub fn write_bell(writer: &mut impl Write) -> io::Result<()> {
    writer.write_all(&[SERVER_BELL])
}
