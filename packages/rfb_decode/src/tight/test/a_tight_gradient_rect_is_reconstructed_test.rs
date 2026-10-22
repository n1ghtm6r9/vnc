use rfb::{ENCODING_TIGHT, Rect};

use crate::test_support::{DecodeHarness, ZlibFixture, gradient_filtered, rgba_pixel};

#[test]
fn test_a_tight_gradient_rect_is_reconstructed() {
    let mut harness = DecodeHarness::new(6, 4);
    let image: [[u8; 3]; 12] = [
        [0, 10, 200],
        [250, 20, 100],
        [5, 30, 0],
        [90, 40, 255],
        [250, 50, 3],
        [10, 60, 254],
        [77, 70, 1],
        [200, 80, 128],
        [13, 90, 64],
        [255, 100, 63],
        [0, 110, 250],
        [33, 120, 7],
    ];
    let rgb: Vec<u8> = image.concat();
    let mut payload = vec![0x40u8, 2];
    payload.extend_from_slice(&ZlibFixture::new().tight_payload(&gradient_filtered(4, &rgb)));
    harness
        .decode(ENCODING_TIGHT, Rect::new(1, 0, 4, 3), &payload)
        .expect("a gradient rect decodes");
    for (index, [red, green, blue]) in image.into_iter().enumerate() {
        let (x, y) = (1 + (index % 4) as u16, (index / 4) as u16);
        assert_eq!(
            harness.pixel(x, y),
            rgba_pixel(red, green, blue),
            "pixel ({x}, {y}) is reconstructed from its differences"
        );
    }
}
