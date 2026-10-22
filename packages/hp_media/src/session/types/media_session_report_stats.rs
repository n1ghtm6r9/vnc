use std::time::Instant;

use super::MediaSession;
use crate::{FrameDecoder, MediaEvent, STATS_INTERVAL};

impl<D: FrameDecoder> MediaSession<D> {
    pub(crate) fn report_stats(&mut self, now: Instant) {
        let Some(stream) = self.phase.stream_mut() else {
            return;
        };
        if now < self.meter.window_start + STATS_INTERVAL {
            return;
        }
        let stats = self.meter.report(now, stream.answer.video.stripes);
        self.effects.events.push(MediaEvent::Stats(stats));
    }
}
