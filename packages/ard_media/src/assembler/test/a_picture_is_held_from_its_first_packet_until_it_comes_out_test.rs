use std::time::{Duration, Instant};

use crate::{
    VideoAssembler,
    test_support::{
        NAL_IDR, NAL_TRAIL, TEST_BASE_SSRC, TestPacket, push_packets, rtp_extension,
        single_unit_packet,
    },
};

#[test]
fn test_a_picture_is_held_from_its_first_packet_until_it_comes_out() {
    let half = |seq: u16, body: &[u8]| TestPacket {
        seq,
        extension: rtp_extension(2, 7),
        ..single_unit_packet(0, 2, NAL_TRAIL, body)
    };
    let mut assembler = VideoAssembler::new(TEST_BASE_SSRC, 1);
    let start = Instant::now();
    let first_half_at = start + Duration::from_millis(10);

    push_packets(
        &mut assembler,
        &[single_unit_packet(0, 1, NAL_IDR, b"key")],
        start,
    );
    let idle = assembler.held_since();
    push_packets(&mut assembler, &[half(100, b"first half")], first_half_at);
    let unfinished = assembler.held_since();
    push_packets(
        &mut assembler,
        &[half(101, b"second half")],
        start + Duration::from_millis(20),
    );
    let finished = assembler.held_since();

    assert_eq!(idle, None, "nothing is held once every picture is out");
    assert_eq!(
        unfinished,
        Some(first_half_at),
        "an unfinished picture is held from its first packet"
    );
    assert_eq!(
        finished, None,
        "a finished picture comes out and is no longer held"
    );
}
