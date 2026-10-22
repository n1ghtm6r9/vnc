use crate::{ConnectOptions, SessionEvent, SessionHandle, launch_session, start_hp_media};

pub fn start_session(
    options: ConnectOptions,
    on_event: Box<dyn FnMut(SessionEvent) + Send>,
) -> SessionHandle {
    launch_session(options, on_event, Box::new(start_hp_media))
}
