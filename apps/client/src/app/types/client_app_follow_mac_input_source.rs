use log::info;
use rfb_session::ClientCommand;

use super::ClientApp;

impl ClientApp {
    pub(super) fn follow_mac_input_source(&mut self, source: &str) {
        let Some(wanted) = self.mac_input_source.report(source) else {
            return;
        };
        info!("[UI] the Mac types with {source}, so it is asked to type with {wanted} for the session");
        self.session.send(ClientCommand::SetInputSource(wanted.to_owned()));
    }
}
