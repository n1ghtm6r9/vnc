use std::time::Instant;

use super::{SessionStats, StatsMeter};
use crate::STATS_INTERVAL;

impl StatsMeter {
    pub(crate) fn record_update(
        &mut self,
        now: Instant,
        bytes: u64,
        last_encoding: i32,
    ) -> Option<SessionStats> {
        self.updates += 1;
        self.bytes += bytes;
        let elapsed = now.saturating_duration_since(self.window_start);
        if elapsed < STATS_INTERVAL {
            return None;
        }
        let seconds = elapsed.as_secs_f64();
        let stats = SessionStats {
            bytes_per_second: (self.bytes as f64 / seconds) as u64,
            updates_per_second: (f64::from(self.updates) / seconds) as f32,
            last_encoding,
        };
        *self = StatsMeter::new(now);
        Some(stats)
    }
}
