use rfb::{ENCODING_ZLIB, Rect};

use crate::test_support::{
    DecodeHarness, finished_zlib_stream, length_prefixed, rgba_pixel, wire_pixel,
};

#[test]
fn test_a_finished_zlib_stream_restarts_for_the_next_rect() {
    let mut harness = DecodeHarness::new(4, 2);
    let first = length_prefixed(&finished_zlib_stream(
        &[wire_pixel(1, 2, 3), wire_pixel(4, 5, 6)].concat(),
    ));
    let second = length_prefixed(&finished_zlib_stream(
        &[wire_pixel(7, 8, 9), wire_pixel(10, 11, 12)].concat(),
    ));
    harness
        .decode(ENCODING_ZLIB, Rect::new(0, 0, 2, 1), &first)
        .expect("the first finished stream decodes");
    harness
        .decode(ENCODING_ZLIB, Rect::new(2, 1, 2, 1), &second)
        .expect("a new stream after a finished one decodes");
    assert_eq!(
        harness.pixel(1, 0),
        rgba_pixel(4, 5, 6),
        "the first rect keeps its pixels"
    );
    assert_eq!(
        harness.pixel(2, 1),
        rgba_pixel(7, 8, 9),
        "the second rect starts with its own first pixel"
    );
    assert_eq!(
        harness.pixel(3, 1),
        rgba_pixel(10, 11, 12),
        "the second rect ends with its own last pixel"
    );
}
