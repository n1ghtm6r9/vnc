use std::time::Instant;

use crate::{
    VideoAssembler,
    test_support::{
        NAL_IDR, NAL_TRAIL, TEST_BASE_SSRC, TestPacket, emitted_dons, push_packets, rtp_extension,
        single_unit_packet,
    },
};

#[test]
fn test_a_cut_or_foreign_packet_is_counted_and_dropped() {
    let mut assembler = VideoAssembler::new(TEST_BASE_SSRC, 2);
    let foreign = TestPacket {
        ssrc: TEST_BASE_SSRC + 2,
        ..single_unit_packet(0, 5, NAL_TRAIL, b"x")
    };
    let no_extension = TestPacket {
        extension: Vec::new(),
        ..single_unit_packet(0, 6, NAL_TRAIL, b"x")
    };
    let no_count = TestPacket {
        extension: rtp_extension(0, 0),
        ..single_unit_packet(0, 7, NAL_TRAIL, b"x")
    };
    let cut = TestPacket {
        payload: vec![2, 1, 0],
        ..single_unit_packet(0, 8, NAL_TRAIL, b"")
    };

    let events = push_packets(
        &mut assembler,
        &[
            foreign,
            no_extension,
            no_count,
            cut,
            single_unit_packet(0, 1, NAL_IDR, b"key"),
            single_unit_packet(0, 1, NAL_IDR, b"key"),
            single_unit_packet(1, 2, NAL_TRAIL, b"next"),
            single_unit_packet(1, 2, NAL_TRAIL, b"next"),
        ],
        Instant::now(),
    );
    let stats = assembler.stats();

    assert_eq!(
        emitted_dons(&events),
        [1, 2],
        "the good pictures come out once each"
    );
    assert_eq!(
        stats.ignored, 2,
        "a stripe that does not exist and a packet without the extension are ignored"
    );
    assert_eq!(
        stats.malformed, 2,
        "a picture of no packets and a cut payload are malformed"
    );
    assert_eq!(stats.duplicates, 2, "a repeated packet is a duplicate");
    assert_eq!(stats.packets, 8, "every packet is counted");
}
