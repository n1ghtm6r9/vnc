use std::time::{Duration, Instant};

use crate::{
    AssemblerEvent, VideoAssembler,
    test_support::{
        NAL_IDR, NAL_TRAIL, TEST_BASE_SSRC, emitted_dons, push_packets, single_unit_packet,
    },
};

#[test]
fn test_a_lost_picture_is_skipped_at_the_next_key_picture() {
    let mut assembler = VideoAssembler::new(TEST_BASE_SSRC, 1);
    let start = Instant::now();

    let before = push_packets(
        &mut assembler,
        &[
            single_unit_packet(0, 1, NAL_IDR, b"key"),
            single_unit_packet(0, 3, NAL_TRAIL, b"after the loss"),
            single_unit_packet(0, 4, NAL_IDR, b"next key"),
            single_unit_packet(0, 5, NAL_TRAIL, b"after the key"),
        ],
        start,
    );
    let mut waited = Vec::new();
    assembler.tick(start + Duration::from_millis(150), &mut waited);
    let mut resumed = Vec::new();
    assembler.tick(start + Duration::from_millis(250), &mut resumed);

    assert_eq!(
        emitted_dons(&before),
        [1],
        "the pictures after the missing one wait for it"
    );
    assert!(waited.is_empty(), "a short wait changes nothing");
    assert_eq!(
        emitted_dons(&resumed),
        [4, 5],
        "after the wait the stream resumes at the next key picture"
    );
    assert!(
        !resumed
            .iter()
            .any(|event| matches!(event, AssemblerEvent::Stalled { .. })),
        "a resync is not a stall"
    );
    assert_eq!(
        assembler.stats().skipped,
        1,
        "the picture that waited in vain is dropped"
    );
}
