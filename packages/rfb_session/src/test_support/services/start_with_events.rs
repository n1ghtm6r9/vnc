use std::sync::mpsc::{self, Receiver};

use crate::{ConnectOptions, SessionEvent, SessionHandle, start_session};

pub fn start_with_events(options: ConnectOptions) -> (SessionHandle, Receiver<SessionEvent>) {
    let (sender, events) = mpsc::channel();
    let handle = start_session(
        options,
        Box::new(move |event| {
            let _ = sender.send(event);
        }),
    );
    (handle, events)
}
