use std::sync::{Arc, Mutex};

use super::{EventCallback, EventSink};

impl EventSink {
    pub(crate) fn new(on_event: EventCallback) -> EventSink {
        EventSink {
            target: Arc::new(Mutex::new(Some(on_event))),
        }
    }
}
