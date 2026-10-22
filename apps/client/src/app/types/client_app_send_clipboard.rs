use log::warn;
use rfb_session::ClientCommand;

use super::{ClientApp, LinkState};
use crate::{mac_line_ends, read_local_clipboard};

impl ClientApp {
    pub(super) fn send_clipboard(&mut self) {
        if self.link != LinkState::Online || !self.settings.clipboard_sync {
            return;
        }
        let local = match read_local_clipboard() {
            Ok(text) => text,
            Err(arboard::Error::ContentNotAvailable) => return,
            Err(error) => {
                warn!("[UI] the Windows clipboard could not be read: {error}");
                return;
            }
        };
        if let Some(text) = self.clipboard.take_changed(local) {
            self.session.send(ClientCommand::ClipboardText(mac_line_ends(&text)));
        }
    }
}
