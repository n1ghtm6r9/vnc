use std::sync::mpsc::Receiver;

use crate::SessionEvent;
use crate::test_support::EVENT_TIMEOUT;

pub fn next_event(events: &Receiver<SessionEvent>) -> SessionEvent {
    events
        .recv_timeout(EVENT_TIMEOUT)
        .expect("the session sends its next event in time")
}
