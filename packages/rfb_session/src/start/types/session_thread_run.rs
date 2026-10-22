use std::sync::mpsc::Receiver;

use log::{info, warn};

use super::SessionThread;
use crate::{Outgoing, SessionEvent};

impl SessionThread {
    pub(crate) fn run(mut self, queue: Receiver<Outgoing>) {
        let outcome = self.serve(queue);
        let _ = self.outgoing.send(Outgoing::Stop);
        let error = outcome.err().filter(|_| !self.control.is_stopped());
        match &error {
            Some(error) => warn!("[SESSION] the session ended: {error}"),
            None => info!("[SESSION] the session ended"),
        }
        self.events.emit_last(SessionEvent::Disconnected { error });
    }
}
