use crate::test_support::{is_near, render_frames};
use crate::{PLAYOUT_TARGET, PlayoutBuffer, SAMPLE_RATE};

#[test]
fn test_nothing_plays_until_the_target_is_buffered() {
    let mut buffer = PlayoutBuffer::new(SAMPLE_RATE);

    buffer.push(&vec![[0.5, 0.5]; PLAYOUT_TARGET - 1]);
    let early = render_frames(&mut buffer, 480);
    buffer.push(&[[0.5, 0.5]]);
    let started = render_frames(&mut buffer, 480);

    assert!(
        early.iter().all(|frame| *frame == [0.0, 0.0]),
        "less sound than the target plays silence"
    );
    assert!(
        started[64..]
            .iter()
            .all(|frame| is_near(*frame, [0.5, 0.5])),
        "the sound plays once the target is buffered"
    );
}
