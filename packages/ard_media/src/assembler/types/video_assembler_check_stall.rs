use std::time::Instant;

use super::{AssemblerEvent, VideoAssembler};
use crate::MAX_BUFFERED_PICTURES;

impl VideoAssembler {
    pub(crate) fn check_stall(&mut self, now: Instant, events: &mut Vec<AssemblerEvent>) {
        if self.stalled {
            return;
        }
        let overflowing = self.pictures.len() > MAX_BUFFERED_PICTURES;
        let waiting_too_long = self.next.is_some()
            && self.pictures.values().any(|picture| {
                picture
                    .done_at
                    .is_some_and(|done| now.saturating_duration_since(done) > self.stall_after)
            });
        if !overflowing && !waiting_too_long {
            return;
        }
        if self.next.is_some() && self.resync_to_key(events) {
            return;
        }
        self.stalled = true;
        let buffered = self.pictures.len();
        self.pictures.clear();
        events.push(AssemblerEvent::Stalled {
            expected_don: self.next,
            buffered,
        });
    }
}
