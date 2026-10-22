use rfb::{ENCODING_TIGHT, Rect};

use crate::test_support::{DecodeHarness, ZlibFixture, rgba_pixel};

#[test]
fn test_a_tight_copy_filter_rect_is_inflated_and_painted() {
    let mut harness = DecodeHarness::new(6, 5);
    let pixels: Vec<u8> = (0..36u8).map(|index| index * 7 + 3).collect();
    let mut payload = vec![0x00u8];
    payload.extend_from_slice(&ZlibFixture::new().tight_payload(&pixels));
    harness
        .decode(ENCODING_TIGHT, Rect::new(1, 1, 4, 3), &payload)
        .expect("a copy filter rect decodes");
    for (index, pixel) in pixels.as_chunks::<3>().0.iter().enumerate() {
        let (x, y) = (1 + (index % 4) as u16, 1 + (index / 4) as u16);
        assert_eq!(
            harness.pixel(x, y),
            rgba_pixel(pixel[0], pixel[1], pixel[2]),
            "pixel {index} lands at ({x}, {y})"
        );
    }
    assert_eq!(
        harness.pixel(0, 0),
        rgba_pixel(0, 0, 0),
        "a pixel outside the rect is untouched"
    );
}
