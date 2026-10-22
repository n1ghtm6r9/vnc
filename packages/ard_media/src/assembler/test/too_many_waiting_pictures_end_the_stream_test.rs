use std::time::Instant;

use crate::{
    AssemblerEvent, MAX_BUFFERED_PICTURES, VideoAssembler,
    test_support::{NAL_TRAIL, TEST_BASE_SSRC, push_packets, single_unit_packet},
};

#[test]
fn test_too_many_waiting_pictures_end_the_stream() {
    let mut assembler = VideoAssembler::new(TEST_BASE_SSRC, 1);
    let packets: Vec<_> = (0..=MAX_BUFFERED_PICTURES as u16)
        .map(|don| single_unit_packet(0, don, NAL_TRAIL, b"no key ever"))
        .collect();

    let events = push_packets(&mut assembler, &packets, Instant::now());

    assert_eq!(
        events,
        [AssemblerEvent::Stalled {
            expected_don: None,
            buffered: MAX_BUFFERED_PICTURES + 1
        }],
        "more waiting pictures than the limit means the stream is lost"
    );
}
