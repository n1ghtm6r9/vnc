use std::{
    sync::{Arc, mpsc},
    thread,
};

use crate::{
    ConnectOptions, EventCallback, EventSink, SessionControl, SessionHandle, SessionThread,
    StartMedia,
};

pub(crate) fn launch_session(
    options: ConnectOptions,
    on_event: EventCallback,
    start_media: StartMedia,
) -> SessionHandle {
    let (outgoing, queue) = mpsc::channel();
    let control = Arc::new(SessionControl::default());
    let session = SessionThread {
        options,
        events: EventSink::new(on_event),
        start_media,
        outgoing: outgoing.clone(),
        control: Arc::clone(&control),
    };
    thread::Builder::new()
        .name("rfb-session".to_owned())
        .spawn(move || session.run(queue))
        .expect("the system could not start the session thread");
    SessionHandle::new(outgoing, control)
}
