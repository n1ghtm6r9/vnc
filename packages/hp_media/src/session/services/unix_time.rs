use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub(crate) fn unix_time() -> Duration {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
}
