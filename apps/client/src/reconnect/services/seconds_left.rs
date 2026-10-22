use std::time::Instant;

pub fn seconds_left(at: Instant, now: Instant) -> u64 {
    let remaining = at.saturating_duration_since(now);
    remaining.as_secs() + u64::from(remaining.subsec_nanos() > 0)
}
