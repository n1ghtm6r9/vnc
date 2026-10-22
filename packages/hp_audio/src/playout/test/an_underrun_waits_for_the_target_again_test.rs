use crate::test_support::{is_near, render_frames};
use crate::{PLAYOUT_TARGET, PlayoutBuffer, SAMPLE_RATE};

#[test]
fn test_an_underrun_waits_for_the_target_again() {
    let mut buffer = PlayoutBuffer::new(SAMPLE_RATE);
    buffer.push(&vec![[0.5, 0.5]; PLAYOUT_TARGET]);

    let drained = render_frames(&mut buffer, PLAYOUT_TARGET + 480);
    buffer.push(&vec![[0.5, 0.5]; PLAYOUT_TARGET / 2]);
    let refilling = render_frames(&mut buffer, 480);
    buffer.push(&vec![[0.5, 0.5]; PLAYOUT_TARGET]);
    let resumed = render_frames(&mut buffer, 480);

    assert!(
        drained[PLAYOUT_TARGET..]
            .iter()
            .all(|frame| *frame == [0.0, 0.0]),
        "an empty buffer plays silence"
    );
    assert!(
        refilling.iter().all(|frame| *frame == [0.0, 0.0]),
        "after running dry the sound waits until the target is buffered again"
    );
    assert!(
        resumed[64..]
            .iter()
            .all(|frame| is_near(*frame, [0.5, 0.5])),
        "the sound comes back once the target is buffered"
    );
}
