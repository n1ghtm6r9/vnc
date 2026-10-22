use std::sync::Mutex;

use crate::test_support::silent_packet;
use crate::{FRAME_SAMPLES, PlayoutBuffer, SAMPLE_RATE, SoundPipeline};

#[test]
fn test_a_lost_packet_keeps_its_time() {
    let mut pipeline = SoundPipeline::new().expect("the sound pipeline starts");
    let buffer = Mutex::new(PlayoutBuffer::new(SAMPLE_RATE));

    for seq in [1, 2, 4, 5, 6, 7, 8, 9] {
        pipeline.push(silent_packet(seq), &buffer);
    }
    let level = buffer.lock().expect("the buffer is not poisoned").level();

    assert!(
        (level - (9 * FRAME_SAMPLES) as f64).abs() < 1.0,
        "a lost packet becomes ten milliseconds of silence, so nine packets' time is {level} frames"
    );
}
