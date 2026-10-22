use crate::test_support::play_steadily;
use crate::{PLAYOUT_TARGET, PlayoutBuffer, SAMPLE_RATE};

#[test]
fn test_a_full_buffer_plays_a_little_faster() {
    let mut buffer = PlayoutBuffer::new(SAMPLE_RATE);
    buffer.push(&vec![[0.25, 0.25]; 2 * PLAYOUT_TARGET]);
    let before = buffer.level();

    play_steadily(&mut buffer, 500);
    let drained = before - buffer.level();

    assert!(
        drained > 400.0,
        "sound that piles up is played a little faster, but only {drained} frames went in five seconds"
    );
    assert!(
        drained < 5.0 * 48_000.0 * 0.004 + 10.0,
        "the speed changes by at most 0.4%, yet {drained} frames went in five seconds"
    );
}
