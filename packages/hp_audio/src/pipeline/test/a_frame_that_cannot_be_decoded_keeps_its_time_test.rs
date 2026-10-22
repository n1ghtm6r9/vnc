use std::sync::Mutex;

use crate::test_support::{TEST_SSRC, silent_packet};
use crate::{FRAME_SAMPLES, PlayoutBuffer, SAMPLE_RATE, SoundPacket, SoundPipeline};

#[test]
fn test_a_frame_that_cannot_be_decoded_keeps_its_time() {
    let mut pipeline = SoundPipeline::new().expect("the sound pipeline starts");
    let buffer = Mutex::new(PlayoutBuffer::new(SAMPLE_RATE));

    pipeline.push(silent_packet(1), &buffer);
    pipeline.push(
        SoundPacket {
            ssrc: TEST_SSRC,
            seq: 2,
            payload: Vec::new(),
        },
        &buffer,
    );
    pipeline.push(silent_packet(3), &buffer);
    let level = buffer.lock().expect("the buffer is not poisoned").level();

    assert!(
        (level - (3 * FRAME_SAMPLES) as f64).abs() < 1.0,
        "a frame that cannot be decoded becomes silence of the same length, so three packets are {level} frames"
    );
}
