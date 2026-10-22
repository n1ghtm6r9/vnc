use super::SessionReader;
use crate::Outgoing;

impl SessionReader {
    pub(super) fn request_update(&self, incremental: bool) {
        self.queue(Outgoing::UpdateRequest {
            incremental,
            rect: self.screen(),
        });
    }
}
