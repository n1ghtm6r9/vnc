use std::time::Instant;

use crate::{
    NAL_TYPE_AGGREGATION, VideoAssembler,
    test_support::{
        NAL_IDR, TEST_BASE_SSRC, TestPacket, emitted_pictures, push_packets, rtp_extension,
    },
};

#[test]
fn test_an_aggregation_packet_splits_into_its_units() {
    let mut payload = vec![NAL_TYPE_AGGREGATION << 1, 1, 0x00, 0x01];
    for unit in [
        &[32 << 1, 1, 0xaa][..],
        &[33 << 1, 1, 0xbb, 0xcc],
        &[NAL_IDR << 1, 1, 0xdd],
    ] {
        payload.extend_from_slice(&(unit.len() as u16).to_be_bytes());
        payload.extend_from_slice(unit);
    }
    let mut cut = payload.clone();
    cut.pop();
    let packet = |seq: u16, payload: Vec<u8>| TestPacket {
        ssrc: TEST_BASE_SSRC,
        seq,
        extension: rtp_extension(1, 0),
        payload,
    };
    let mut assembler = VideoAssembler::new(TEST_BASE_SSRC, 1);

    let events = push_packets(
        &mut assembler,
        &[packet(1, cut), packet(2, payload)],
        Instant::now(),
    );
    let pictures = emitted_pictures(&events);

    assert_eq!(
        pictures.len(),
        1,
        "the well-formed aggregation packet makes one picture"
    );
    assert_eq!(
        pictures[0].data,
        [
            0,
            0,
            0,
            1,
            64,
            1,
            0xaa,
            0,
            0,
            0,
            1,
            66,
            1,
            0xbb,
            0xcc,
            0,
            0,
            0,
            1,
            NAL_IDR << 1,
            1,
            0xdd
        ],
        "every unit gets a start code"
    );
    assert_eq!(
        assembler.stats().malformed,
        1,
        "the cut aggregation packet is counted as malformed"
    );
}
