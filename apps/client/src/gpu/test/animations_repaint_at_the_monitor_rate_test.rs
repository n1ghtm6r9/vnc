use std::time::Duration;

use crate::frame_interval;

#[test]
fn test_animations_repaint_at_the_monitor_rate() {
    assert_eq!(
        frame_interval(Some(120_000)),
        Duration::from_secs_f64(1.0 / 120.0),
        "a 120 Hz monitor gets a frame every 1/120 s"
    );
    assert_eq!(
        frame_interval(None),
        Duration::from_secs_f64(1.0 / 60.0),
        "a monitor without a known rate is taken as 60 Hz"
    );
    assert_eq!(
        frame_interval(Some(0)),
        Duration::from_secs_f64(1.0 / 60.0),
        "a rate of zero is taken as 60 Hz"
    );
}
