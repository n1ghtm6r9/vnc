use std::io;

use rfb::{write_apple_clipboard, write_client_cut_text};

use super::SessionWriter;
use crate::latin1_bytes;

impl SessionWriter {
    pub(super) fn write_clipboard(&mut self, text: String) -> io::Result<()> {
        if self.apple_key.is_none() {
            return write_client_cut_text(&mut self.output, &latin1_bytes(&text));
        }
        write_apple_clipboard(&mut self.output, self.clipboard_session, &text)?;
        self.clipboard_text = Some(text);
        Ok(())
    }
}
