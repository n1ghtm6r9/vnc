use std::sync::{
    Arc,
    mpsc::{Receiver, Sender},
};

use super::SessionHandle;
use crate::{Outgoing, SessionControl};

impl SessionHandle {
    pub(crate) fn new(
        outgoing: Sender<Outgoing>,
        control: Arc<SessionControl>,
        writer_finished: Receiver<()>,
    ) -> SessionHandle {
        SessionHandle {
            outgoing,
            control,
            writer_finished,
        }
    }
}
