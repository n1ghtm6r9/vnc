use std::time::Duration;

use crate::FALLBACK_REFRESH_HERTZ;

pub fn frame_interval(refresh_millihertz: Option<u32>) -> Duration {
    let hertz = refresh_millihertz
        .filter(|&millihertz| millihertz >= 1_000)
        .map_or(FALLBACK_REFRESH_HERTZ, |millihertz| f64::from(millihertz) / 1_000.0);
    Duration::from_secs_f64(1.0 / hertz)
}
