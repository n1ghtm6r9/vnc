use std::sync::{Arc, mpsc::Sender};

use super::SessionHandle;
use crate::{Outgoing, SessionControl};

impl SessionHandle {
    pub(crate) fn new(outgoing: Sender<Outgoing>, control: Arc<SessionControl>) -> SessionHandle {
        SessionHandle { outgoing, control }
    }
}
