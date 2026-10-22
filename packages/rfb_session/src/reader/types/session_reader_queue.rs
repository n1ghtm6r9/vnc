use super::SessionReader;
use crate::Outgoing;

impl SessionReader {
    pub(super) fn queue(&self, message: Outgoing) {
        let _ = self.requests.send(message);
    }
}
