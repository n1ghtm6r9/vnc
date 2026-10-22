use super::SessionReader;
use crate::Outgoing;

impl SessionReader {
    pub(super) fn renew_pushed_updates(&self) {
        self.queue(Outgoing::AutoUpdate {
            rect: self.screen(),
        });
    }
}
