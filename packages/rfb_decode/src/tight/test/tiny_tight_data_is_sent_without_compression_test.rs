use rfb::{ENCODING_TIGHT, Rect};

use crate::test_support::{DecodeHarness, rgba_pixel};

#[test]
fn test_tiny_tight_data_is_sent_without_compression() {
    let mut harness = DecodeHarness::new(5, 3);
    let payload = [0x00u8, 10, 20, 30, 40, 50, 60, 70, 80, 90];
    harness
        .decode(ENCODING_TIGHT, Rect::new(1, 1, 3, 1), &payload)
        .expect("nine bytes of pixel data are read as they are");
    assert_eq!(
        harness.pixel(1, 1),
        rgba_pixel(10, 20, 30),
        "the first raw pixel"
    );
    assert_eq!(
        harness.pixel(2, 1),
        rgba_pixel(40, 50, 60),
        "the second raw pixel"
    );
    assert_eq!(
        harness.pixel(3, 1),
        rgba_pixel(70, 80, 90),
        "the third raw pixel"
    );
}
