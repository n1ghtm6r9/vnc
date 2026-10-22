use std::time::{Duration, Instant};

use crate::seconds_left;

#[test]
fn test_the_countdown_rounds_up_to_whole_seconds() {
    let now = Instant::now();

    assert_eq!(
        seconds_left(now + Duration::from_millis(1_200), now),
        2,
        "1.2 seconds left reads as 2 seconds"
    );
    assert_eq!(
        seconds_left(now + Duration::from_secs(1), now),
        1,
        "exactly one second left reads as 1 second"
    );
    assert_eq!(
        seconds_left(now, now + Duration::from_secs(3)),
        0,
        "a moment already passed has no seconds left"
    );
}
