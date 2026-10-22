use std::sync::Mutex;

use crate::test_support::silent_packet;
use crate::{FRAME_SAMPLES, PlayoutBuffer, SAMPLE_RATE, SoundPipeline};

#[test]
fn test_decoded_sound_reaches_the_playout_buffer() {
    let mut pipeline = SoundPipeline::new().expect("the sound pipeline starts");
    let buffer = Mutex::new(PlayoutBuffer::new(SAMPLE_RATE));

    for seq in 1..=3 {
        pipeline.push(silent_packet(seq), &buffer);
    }
    let level = buffer.lock().expect("the buffer is not poisoned").level();

    assert!(
        (level - (3 * FRAME_SAMPLES) as f64).abs() < 1.0,
        "three packets give thirty milliseconds of sound, not {level} frames"
    );
}
