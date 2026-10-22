use std::time::{Duration, Instant};

use crate::{
    AssemblerEvent, VideoAssembler,
    test_support::{NAL_IDR, NAL_TRAIL, TEST_BASE_SSRC, push_packets, single_unit_packet},
};

#[test]
fn test_a_lost_picture_with_no_key_picture_after_it_stalls() {
    let mut assembler = VideoAssembler::new(TEST_BASE_SSRC, 1);
    let start = Instant::now();
    push_packets(
        &mut assembler,
        &[
            single_unit_packet(0, 1, NAL_IDR, b"key"),
            single_unit_packet(0, 3, NAL_TRAIL, b"after the loss"),
        ],
        start,
    );

    let mut events = Vec::new();
    assembler.tick(start + Duration::from_millis(250), &mut events);
    let after = push_packets(
        &mut assembler,
        &[single_unit_packet(0, 2, NAL_TRAIL, b"too late")],
        start,
    );

    assert_eq!(
        events,
        [AssemblerEvent::Stalled {
            expected_don: Some(2),
            buffered: 1
        }],
        "without a key picture to land on the stream stops at the missing picture"
    );
    assert!(after.is_empty(), "nothing comes out after a stall");
    assert_eq!(
        assembler.stats().ignored,
        1,
        "a packet after the stall is ignored"
    );
}
