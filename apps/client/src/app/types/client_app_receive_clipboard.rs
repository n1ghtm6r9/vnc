use log::warn;

use super::ClientApp;
use crate::{windows_line_ends, write_local_clipboard};

impl ClientApp {
    pub(super) fn receive_clipboard(&mut self, text: &str) {
        if !self.settings.clipboard_sync {
            return;
        }
        let text = windows_line_ends(text);
        match write_local_clipboard(&text) {
            Ok(()) => self.clipboard.note_synced(&text),
            Err(error) => warn!("[UI] the clipboard text from the Mac could not be stored: {error}"),
        }
    }
}
