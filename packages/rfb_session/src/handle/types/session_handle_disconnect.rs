use super::SessionHandle;
use crate::Outgoing;

impl SessionHandle {
    pub fn disconnect(&self) {
        self.control.stop();
        let _ = self.outgoing.send(Outgoing::Stop);
    }
}
