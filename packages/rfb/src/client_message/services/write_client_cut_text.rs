use std::io::{self, Write};

use crate::{CLIENT_CUT_TEXT, write_text};

pub fn write_client_cut_text(writer: &mut impl Write, text: &[u8]) -> io::Result<()> {
    writer.write_all(&[CLIENT_CUT_TEXT, 0, 0, 0])?;
    write_text(writer, text)
}
