use std::time::Instant;

use crate::{
    NAL_TYPE_FRAGMENT, VideoAssembler,
    test_support::{
        NAL_IDR, TEST_BASE_SSRC, TestPacket, emitted_pictures, push_packets, rtp_extension,
    },
};

#[test]
fn test_a_fragmented_unit_is_rebuilt_from_its_pieces() {
    let piece = |seq: u16, flags: u8, body: &[u8]| {
        let mut payload = vec![(NAL_TYPE_FRAGMENT << 1) | 1, 1, flags | NAL_IDR, 0x00, 0x05];
        payload.extend_from_slice(body);
        TestPacket {
            ssrc: TEST_BASE_SSRC,
            seq,
            extension: rtp_extension(3, 1),
            payload,
        }
    };
    let mut assembler = VideoAssembler::new(TEST_BASE_SSRC, 1);

    let events = push_packets(
        &mut assembler,
        &[
            piece(65535, 0x00, b"middle-"),
            piece(0, 0x40, b"end"),
            piece(65534, 0x80, b"start-"),
        ],
        Instant::now(),
    );
    let pictures = emitted_pictures(&events);

    assert_eq!(pictures.len(), 1, "the three pieces make one picture");
    assert_eq!(
        pictures[0].data,
        [
            &[0, 0, 0, 1, (NAL_IDR << 1) | 1, 1][..],
            b"start-middle-end"
        ]
        .concat(),
        "the unit gets its own header back with the layer bit kept, and its pieces in sequence order across the wrap"
    );
    assert!(pictures[0].key, "a rebuilt IDR unit makes a key picture");
}
