use rfb::{ENCODING_COPY_RECT, Rect};

use crate::test_support::{DecodeHarness, rgba_pixel};

#[test]
fn test_copy_rect_overlapping_down_and_right_keeps_the_original_pixels() {
    let mut harness = DecodeHarness::new(6, 6);
    let numbered: Vec<u32> = (0..36).map(|index| rgba_pixel(index, 0, 0)).collect();
    harness
        .framebuffer
        .put_pixels(Rect::new(0, 0, 6, 6), &numbered);
    let payload = [0u16.to_be_bytes(), 0u16.to_be_bytes()].concat();
    harness
        .decode(ENCODING_COPY_RECT, Rect::new(1, 1, 4, 4), &payload)
        .expect("an overlapping copy decodes");
    for y in 1..5u16 {
        for x in 1..5u16 {
            let original = numbered[usize::from(y - 1) * 6 + usize::from(x - 1)];
            assert_eq!(
                harness.pixel(x, y),
                original,
                "pixel ({x}, {y}) holds the original pixel one step up and left"
            );
        }
    }
}
