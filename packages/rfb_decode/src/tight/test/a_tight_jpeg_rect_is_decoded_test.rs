use rfb::{ENCODING_TIGHT, Rect};

use crate::test_support::{
    DecodeHarness, compact_length, jpeg_of_rgb, pixel_is_close_to, rgba_pixel,
};

#[test]
fn test_a_tight_jpeg_rect_is_decoded() {
    let mut harness = DecodeHarness::new(24, 24);
    let mut rgb = Vec::new();
    for _ in 0..16 {
        rgb.extend([[220u8, 30, 30]; 8].concat());
        rgb.extend([[30u8, 30, 220]; 8].concat());
    }
    let jpeg = jpeg_of_rgb(16, 16, &rgb);
    let mut payload = vec![0x90u8];
    payload.extend_from_slice(&compact_length(jpeg.len()));
    payload.extend_from_slice(&jpeg);
    harness
        .decode(ENCODING_TIGHT, Rect::new(2, 3, 16, 16), &payload)
        .expect("a JPEG rect decodes");
    assert!(
        pixel_is_close_to(harness.pixel(2, 3), [220, 30, 30], 12),
        "the top left pixel is red"
    );
    assert!(
        pixel_is_close_to(harness.pixel(7, 18), [220, 30, 30], 12),
        "a pixel in the left half of the bottom row is red"
    );
    assert!(
        pixel_is_close_to(harness.pixel(12, 3), [30, 30, 220], 12),
        "a pixel in the right half of the top row is blue"
    );
    assert!(
        pixel_is_close_to(harness.pixel(17, 18), [30, 30, 220], 12),
        "the bottom right pixel is blue"
    );
    assert_eq!(
        harness.pixel(1, 3),
        rgba_pixel(0, 0, 0),
        "the pixel left of the rect is untouched"
    );
}
