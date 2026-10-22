use rfb_session::ClientCommand;

use super::SessionSlot;

impl SessionSlot {
    pub fn send(&self, command: ClientCommand) {
        if let Some(handle) = &self.handle {
            handle.send(command);
        }
    }
}
