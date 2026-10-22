use std::time::Instant;

use super::StatsMeter;

impl StatsMeter {
    pub(crate) fn new(now: Instant) -> StatsMeter {
        StatsMeter {
            window_start: now,
            updates: 0,
            bytes: 0,
        }
    }
}
