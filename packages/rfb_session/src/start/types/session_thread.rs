use std::sync::{Arc, mpsc::Sender};

use crate::{ConnectOptions, EventSink, Outgoing, SessionControl, StartMedia};

pub(crate) struct SessionThread {
    pub(crate) options: ConnectOptions,
    pub(crate) events: EventSink,
    pub(crate) start_media: StartMedia,
    pub(crate) outgoing: Sender<Outgoing>,
    pub(crate) control: Arc<SessionControl>,
    pub(crate) writer_finished: Sender<()>,
}
