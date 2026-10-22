use std::time::{Duration, Instant};

use crate::frame_wait_until;

#[test]
fn test_a_redraw_within_one_frame_of_the_last_waits_for_the_next_frame() {
    let last_frame = Instant::now();
    let interval = Duration::from_millis(7);

    let too_soon = frame_wait_until(Some(last_frame), interval, last_frame + Duration::from_millis(2));
    let on_time = frame_wait_until(Some(last_frame), interval, last_frame + interval);
    let first = frame_wait_until(None, interval, last_frame);

    assert_eq!(
        too_soon,
        Some(last_frame + interval),
        "a pointer move 2 ms after a frame is drawn with the next frame, one interval after the last"
    );
    assert_eq!(on_time, None, "once a whole frame interval has passed the redraw happens at once");
    assert_eq!(first, None, "the first frame is drawn at once");
}
