use std::time::Instant;

use super::MediaSession;
use crate::FrameDecoder;

impl<D: FrameDecoder> MediaSession<D> {
    pub(crate) fn tick(&mut self, now: Instant) {
        if let Some(failure) = self.phase.expired(now) {
            self.fail(failure);
            return;
        }
        self.forget_unanswered_resize(now);
        if let Some(stream) = self.phase.stream_mut() {
            stream.assembler.tick(now, &mut self.assembled);
        }
        self.settle(now);
        if let Some(failure) = self.phase.silence(now) {
            self.glitch(failure, now);
        }
        self.send_wanted_display(now);
        self.send_feedback(now);
        self.report_stats(now);
    }
}
