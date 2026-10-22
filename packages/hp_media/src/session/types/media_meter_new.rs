use std::time::Instant;

use super::MediaMeter;

impl MediaMeter {
    pub(crate) fn new(now: Instant) -> MediaMeter {
        MediaMeter {
            window_start: now,
            bytes: 0,
            pictures: 0,
        }
    }
}
