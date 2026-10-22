use crate::test_support::{is_near, render_frames};
use crate::{PLAYOUT_LIMIT, PLAYOUT_TARGET, PlayoutBuffer, SAMPLE_RATE};

#[test]
fn test_too_much_sound_drops_back_to_the_target() {
    let mut buffer = PlayoutBuffer::new(SAMPLE_RATE);

    buffer.push(&vec![[0.25, 0.25]; PLAYOUT_LIMIT]);
    buffer.push(&vec![[0.75, 0.75]; 480]);
    let level = buffer.level();
    let played = render_frames(&mut buffer, PLAYOUT_TARGET - 200);

    assert!(
        (level - PLAYOUT_TARGET as f64).abs() < 1.0,
        "more sound than the limit drops back to the target, not to {level} frames"
    );
    assert!(
        played[PLAYOUT_TARGET - 300..]
            .iter()
            .all(|frame| is_near(*frame, [0.75, 0.75])),
        "the oldest sound goes and the newest stays"
    );
}
