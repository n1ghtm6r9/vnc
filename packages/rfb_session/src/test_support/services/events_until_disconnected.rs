use std::sync::mpsc::{Receiver, RecvTimeoutError};

use crate::SessionEvent;
use crate::test_support::{EVENT_TIMEOUT, next_event};

pub fn events_until_disconnected(events: &Receiver<SessionEvent>) -> Vec<SessionEvent> {
    let mut seen = Vec::new();
    loop {
        let event = next_event(events);
        let finished = matches!(event, SessionEvent::Disconnected { .. });
        seen.push(event);
        if finished {
            break;
        }
    }
    let after = events.recv_timeout(EVENT_TIMEOUT);
    assert!(
        matches!(after, Err(RecvTimeoutError::Disconnected)),
        "nothing follows Disconnected and the session lets go of its callback, got {after:?}"
    );
    seen
}
