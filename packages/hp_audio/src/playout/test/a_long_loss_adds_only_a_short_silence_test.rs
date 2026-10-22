use crate::{FRAME_SAMPLES, MAX_CONCEALED_FRAMES, PlayoutBuffer, SAMPLE_RATE};

#[test]
fn test_a_long_loss_adds_only_a_short_silence() {
    let mut buffer = PlayoutBuffer::new(SAMPLE_RATE);

    buffer.conceal(1000);

    assert!(
        (buffer.level() - (MAX_CONCEALED_FRAMES * FRAME_SAMPLES) as f64).abs() < 1.0,
        "a long loss stands in for at most a tenth of a second of silence, not {} frames",
        buffer.level()
    );
}
