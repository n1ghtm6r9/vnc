use std::time::Instant;

use crate::{
    VideoAssembler,
    test_support::{
        NAL_IDR, NAL_TRAIL, TEST_BASE_SSRC, annex_b_unit, emitted_pictures, push_packets,
        single_unit_packet,
    },
};

#[test]
fn test_pictures_come_out_in_decoding_order_from_the_first_key_picture() {
    let mut assembler = VideoAssembler::new(TEST_BASE_SSRC, 4);

    let events = push_packets(
        &mut assembler,
        &[
            single_unit_packet(0, 9, NAL_TRAIL, b"before"),
            single_unit_packet(2, 12, NAL_TRAIL, b"third"),
            single_unit_packet(1, 11, NAL_TRAIL, b"second"),
            single_unit_packet(0, 10, NAL_IDR, b"key"),
            single_unit_packet(3, 13, NAL_TRAIL, b"fourth"),
        ],
        Instant::now(),
    );
    let pictures = emitted_pictures(&events);

    assert_eq!(
        pictures
            .iter()
            .map(|picture| picture.don)
            .collect::<Vec<_>>(),
        [10, 11, 12, 13],
        "emission starts at the key picture and follows the decoding order, not the arrival order"
    );
    assert_eq!(
        pictures
            .iter()
            .map(|picture| picture.stripe)
            .collect::<Vec<_>>(),
        [0, 1, 2, 3],
        "each picture knows its stripe from its source"
    );
    assert!(
        pictures[0].key && !pictures[1].key,
        "only the IDR picture is a key picture"
    );
    assert_eq!(
        pictures[0].data,
        annex_b_unit(NAL_IDR, b"key"),
        "a picture is its NAL units with start codes"
    );
    assert_eq!(
        pictures[0].frame, 7,
        "the frame number comes from the header extension"
    );
    assert_eq!(
        assembler.stats().discarded,
        1,
        "the picture before the first key picture is dropped"
    );
}
