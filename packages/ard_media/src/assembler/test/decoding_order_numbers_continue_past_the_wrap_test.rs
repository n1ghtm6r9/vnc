use std::time::Instant;

use crate::{
    VideoAssembler,
    test_support::{
        NAL_IDR, NAL_TRAIL, TEST_BASE_SSRC, emitted_dons, push_packets, single_unit_packet,
    },
};

#[test]
fn test_decoding_order_numbers_continue_past_the_wrap() {
    let mut assembler = VideoAssembler::new(TEST_BASE_SSRC, 1);

    let events = push_packets(
        &mut assembler,
        &[
            single_unit_packet(0, 65534, NAL_IDR, b"a"),
            single_unit_packet(0, 0, NAL_TRAIL, b"c"),
            single_unit_packet(0, 65535, NAL_TRAIL, b"b"),
            single_unit_packet(0, 1, NAL_TRAIL, b"d"),
        ],
        Instant::now(),
    );

    assert_eq!(
        emitted_dons(&events),
        [65534, 65535, 65536, 65537],
        "the numbers keep growing past 65535 and the wire value is the low 16 bits"
    );
}
