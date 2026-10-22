use std::io;

use rfb::write_apple_clipboard;

use super::SessionWriter;

impl SessionWriter {
    pub(super) fn resend_clipboard(&mut self) -> io::Result<()> {
        match &self.clipboard_text {
            Some(text) => write_apple_clipboard(&mut self.output, self.clipboard_session, text),
            None => Ok(()),
        }
    }
}
