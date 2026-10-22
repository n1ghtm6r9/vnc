use super::SessionHandle;
use crate::{ClientCommand, Outgoing};

impl SessionHandle {
    pub fn send(&self, command: ClientCommand) {
        let _ = self.outgoing.send(Outgoing::Command(command));
    }
}
