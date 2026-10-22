use rfb::{ENCODING_APPLE_MEDIA_STREAM, Rect};

use crate::test_support::DecodeHarness;

#[test]
fn test_a_truncated_media_rect_is_an_error() {
    let mut harness = DecodeHarness::new(4, 4);

    let outcome = harness.decode(
        ENCODING_APPLE_MEDIA_STREAM,
        Rect::new(0, 0, 0, 0),
        &[0, 5, 1, 2],
    );

    assert!(
        outcome.is_err(),
        "a media rect shorter than its length is refused, got {outcome:?}"
    );
}
