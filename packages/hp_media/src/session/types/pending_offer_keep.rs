use super::{Datagram, PendingOffer};
use crate::EARLY_DATAGRAM_LIMIT;

impl PendingOffer {
    pub(crate) fn keep(&mut self, datagram: Datagram) {
        if self.early.len() >= EARLY_DATAGRAM_LIMIT {
            self.early.pop_front();
        }
        self.early.push_back(datagram);
    }
}
