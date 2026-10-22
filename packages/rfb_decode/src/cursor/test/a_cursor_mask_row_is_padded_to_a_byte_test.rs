use rfb::{ENCODING_CURSOR, Rect};

use crate::RectOutcome;
use crate::test_support::{DecodeHarness, wire_pixel};

#[test]
fn test_a_cursor_mask_row_is_padded_to_a_byte() {
    let mut harness = DecodeHarness::new(4, 4);
    let mut payload = Vec::new();
    for index in 0..18u8 {
        payload.extend_from_slice(&wire_pixel(index * 10, index * 10 + 1, index * 10 + 2));
    }
    payload.extend_from_slice(&[0xFF, 0x80, 0x00, 0x80]);
    let outcome = harness
        .decode(ENCODING_CURSOR, Rect::new(0, 0, 9, 2), &payload)
        .expect("a nine pixel wide cursor decodes");
    let RectOutcome::Cursor(shape) = outcome else {
        panic!("a cursor rect gives a cursor outcome, got {outcome:?}");
    };
    assert_eq!(
        shape.rgba.len(),
        9 * 2 * 4,
        "the shape holds four bytes for each of its pixels"
    );
    let alpha = |x: usize, y: usize| shape.rgba[(y * 9 + x) * 4 + 3];
    assert_eq!(
        alpha(0, 0),
        255,
        "the first pixel of the first row is visible"
    );
    assert_eq!(
        alpha(8, 0),
        255,
        "the ninth pixel of the first row comes from the second mask byte"
    );
    assert_eq!(
        alpha(0, 1),
        0,
        "the first pixel of the second row is hidden"
    );
    assert_eq!(
        alpha(7, 1),
        0,
        "the eighth pixel of the second row is hidden"
    );
    assert_eq!(
        alpha(8, 1),
        255,
        "the ninth pixel of the second row is visible"
    );
    assert_eq!(
        &shape.rgba[(9 + 8) * 4..(9 + 8) * 4 + 4],
        &[170, 171, 172, 255],
        "a visible pixel keeps its colour"
    );
}
