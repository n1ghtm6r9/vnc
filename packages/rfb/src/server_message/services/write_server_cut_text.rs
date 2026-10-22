use std::io::{self, Write};

use crate::{SERVER_CUT_TEXT, write_text};

pub fn write_server_cut_text(writer: &mut impl Write, text: &[u8]) -> io::Result<()> {
    writer.write_all(&[SERVER_CUT_TEXT, 0, 0, 0])?;
    write_text(writer, text)
}
