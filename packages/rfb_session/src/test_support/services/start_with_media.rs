use std::sync::mpsc::{self, Receiver};

use crate::{ConnectOptions, SessionEvent, SessionHandle, StartMedia, launch_session};

pub fn start_with_media(
    options: ConnectOptions,
    start_media: StartMedia,
) -> (SessionHandle, Receiver<SessionEvent>) {
    let (sender, events) = mpsc::channel();
    let handle = launch_session(
        options,
        Box::new(move |event| {
            let _ = sender.send(event);
        }),
        start_media,
    );
    (handle, events)
}
