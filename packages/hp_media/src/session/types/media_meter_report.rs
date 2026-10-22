use std::time::Instant;

use super::MediaMeter;
use crate::MediaStats;

impl MediaMeter {
    pub(crate) fn report(&mut self, now: Instant, stripes: u32) -> MediaStats {
        let seconds = now
            .saturating_duration_since(self.window_start)
            .as_secs_f64();
        let stats = MediaStats {
            bytes_per_second: (self.bytes as f64 / seconds).round() as u64,
            frames_per_second: (self.pictures as f64 / f64::from(stripes.max(1)) / seconds) as f32,
        };
        *self = MediaMeter::new(now);
        stats
    }
}
