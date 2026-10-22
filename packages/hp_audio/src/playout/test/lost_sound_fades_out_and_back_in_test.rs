use crate::test_support::render_frames;
use crate::{FRAME_SAMPLES, PLAYOUT_TARGET, PlayoutBuffer, SAMPLE_RATE};

#[test]
fn test_lost_sound_fades_out_and_back_in() {
    let mut buffer = PlayoutBuffer::new(SAMPLE_RATE);
    buffer.push(&vec![[0.5, 0.5]; PLAYOUT_TARGET]);

    buffer.conceal(1);
    buffer.push(&vec![[0.5, 0.5]; 2 * FRAME_SAMPLES]);
    let played = render_frames(&mut buffer, PLAYOUT_TARGET + 3 * FRAME_SAMPLES - 100);
    let gap = &played[PLAYOUT_TARGET + 100..PLAYOUT_TARGET + FRAME_SAMPLES - 100];
    let largest_jump = played[64..]
        .windows(2)
        .map(|pair| (pair[1][0] - pair[0][0]).abs())
        .fold(0.0, f32::max);

    assert!(
        gap.iter().all(|frame| frame[0].abs() < 1e-3),
        "a lost frame plays as silence"
    );
    assert!(
        played
            .last()
            .is_some_and(|frame| (frame[0] - 0.5).abs() < 1e-3),
        "the sound after the loss comes back in full"
    );
    assert!(
        largest_jump < 0.02,
        "the sound fades into and out of the gap without a click, but it jumps by {largest_jump}"
    );
}
