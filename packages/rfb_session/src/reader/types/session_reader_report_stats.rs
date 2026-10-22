use std::time::Instant;

use super::SessionReader;
use crate::SessionEvent;

impl SessionReader {
    pub(super) fn report_stats(&mut self, on_event: &mut dyn FnMut(SessionEvent)) {
        let bytes = self.input.get_mut().take_count();
        if let Some(stats) = self
            .meter
            .record_update(Instant::now(), bytes, self.last_encoding)
        {
            on_event(SessionEvent::Stats(stats));
        }
    }
}
