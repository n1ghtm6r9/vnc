use std::time::Duration;

use crate::RECONNECT_DELAYS;

pub fn reconnect_delay(attempt: usize) -> Duration {
    RECONNECT_DELAYS[attempt.min(RECONNECT_DELAYS.len() - 1)]
}
