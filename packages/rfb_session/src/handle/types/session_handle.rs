use std::sync::{
    Arc,
    mpsc::{Receiver, Sender},
};

use crate::{Outgoing, SessionControl};

#[derive(Debug)]
pub struct SessionHandle {
    pub(super) outgoing: Sender<Outgoing>,
    pub(super) control: Arc<SessionControl>,
    pub(super) writer_finished: Receiver<()>,
}
