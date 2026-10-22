use crate::test_support::{play_steadily, render_frames};
use crate::{PLAYOUT_TARGET, PlayoutBuffer, SAMPLE_RATE};

#[test]
fn test_a_low_buffer_plays_a_little_slower() {
    let mut buffer = PlayoutBuffer::new(SAMPLE_RATE);
    buffer.push(&vec![[0.25, 0.25]; PLAYOUT_TARGET]);
    render_frames(&mut buffer, PLAYOUT_TARGET / 2);
    let before = buffer.level();

    play_steadily(&mut buffer, 500);
    let gained = buffer.level() - before;

    assert!(
        gained > 300.0,
        "a buffer below the target is played a little slower, but it gained only {gained} frames in five seconds"
    );
    assert!(
        gained < 5.0 * 48_000.0 * 0.004 + 10.0,
        "the speed changes by at most 0.4%, yet it gained {gained} frames in five seconds"
    );
}
