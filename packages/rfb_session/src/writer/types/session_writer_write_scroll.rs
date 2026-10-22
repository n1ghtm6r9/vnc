use std::io;

use log::debug;
use rfb::{AppleScroll, write_apple_scroll};

use super::SessionWriter;

impl SessionWriter {
    pub(super) fn write_scroll(&mut self, scroll: AppleScroll) -> io::Result<()> {
        if self.apple_key.is_none() {
            debug!("[SESSION] plain RFB has no scroll gesture, the scroll is dropped");
            return Ok(());
        }
        write_apple_scroll(&mut self.output, scroll)
    }
}
