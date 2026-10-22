use std::sync::PoisonError;

use super::{EventSink, SessionEvent};

impl EventSink {
    pub(crate) fn emit_last(&self, event: SessionEvent) {
        let mut target = self.target.lock().unwrap_or_else(PoisonError::into_inner);
        if let Some(mut on_event) = target.take() {
            on_event(event);
        }
    }
}
