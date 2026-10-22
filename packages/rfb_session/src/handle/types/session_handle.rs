use std::sync::{Arc, mpsc::Sender};

use crate::{Outgoing, SessionControl};

#[derive(Debug)]
pub struct SessionHandle {
    pub(super) outgoing: Sender<Outgoing>,
    pub(super) control: Arc<SessionControl>,
}
