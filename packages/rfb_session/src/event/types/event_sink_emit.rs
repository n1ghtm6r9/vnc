use std::sync::PoisonError;

use super::{EventSink, SessionEvent};

impl EventSink {
    pub(crate) fn emit(&self, event: SessionEvent) {
        let mut target = self.target.lock().unwrap_or_else(PoisonError::into_inner);
        if let Some(on_event) = target.as_mut() {
            on_event(event);
        }
    }
}
