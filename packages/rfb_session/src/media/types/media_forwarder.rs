use std::sync::{Arc, atomic::AtomicBool, mpsc::Sender};

use crate::{EventSink, Outgoing};

pub(crate) struct MediaForwarder {
    pub(crate) events: EventSink,
    pub(crate) outgoing: Sender<Outgoing>,
    pub(crate) streaming: Arc<AtomicBool>,
}
