use log::info;
use rfb_session::ClientCommand;

use super::ClientApp;

impl ClientApp {
    pub(super) fn give_back_mac_input_source(&mut self) {
        let Some(own) = self.mac_input_source.give_back() else {
            return;
        };
        info!("[UI] the Mac gets its input source {own} back");
        self.session.send(ClientCommand::SetInputSource(own));
    }
}
