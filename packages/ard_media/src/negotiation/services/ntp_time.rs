use std::time::Duration;

use crate::NTP_UNIX_OFFSET;

pub fn ntp_time(since_unix_epoch: Duration) -> u64 {
    let millis = since_unix_epoch.as_millis();
    let seconds = (millis / 1000) as u64 + NTP_UNIX_OFFSET;
    let fraction = ((millis % 1000) << 32) / 1000;
    (seconds << 32) | fraction as u64
}
