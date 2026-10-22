use std::io;

use rfb::{write_apple_auto_update, write_framebuffer_update_request};

use super::SessionWriter;

impl SessionWriter {
    pub(super) fn refresh_screen(&mut self) -> io::Result<()> {
        if !self.media_streaming() {
            write_framebuffer_update_request(&mut self.output, false, self.screen)?;
        }
        if self.apple_key.is_some() {
            write_apple_auto_update(&mut self.output, self.screen)?;
        }
        Ok(())
    }
}
