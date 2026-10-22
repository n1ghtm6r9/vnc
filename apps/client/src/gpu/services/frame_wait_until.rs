use std::time::{Duration, Instant};

pub fn frame_wait_until(last_frame: Option<Instant>, interval: Duration, now: Instant) -> Option<Instant> {
    let due = last_frame?.checked_add(interval)?;
    (now < due).then_some(due)
}
