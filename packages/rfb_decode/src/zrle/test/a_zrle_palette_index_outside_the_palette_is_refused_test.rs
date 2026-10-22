use rfb::{ENCODING_ZRLE, Rect};

use crate::DecodeError;
use crate::test_support::{DecodeHarness, ZlibFixture, compact_pixel};

#[test]
fn test_a_zrle_palette_index_outside_the_palette_is_refused() {
    let red = compact_pixel(255, 0, 0);
    let green = compact_pixel(0, 255, 0);
    let blue = compact_pixel(0, 0, 255);
    let mut packed = vec![3u8];
    for colour in [red, green, blue] {
        packed.extend_from_slice(&colour);
    }
    packed.extend_from_slice(&[0b0001_1011, 0, 0, 0]);
    let mut single_pixel_index = vec![130u8];
    single_pixel_index.extend_from_slice(&red);
    single_pixel_index.extend_from_slice(&green);
    single_pixel_index.extend_from_slice(&[2, 0, 0, 0, 0, 0, 0, 0]);
    let mut run_index = vec![130u8];
    run_index.extend_from_slice(&red);
    run_index.extend_from_slice(&green);
    run_index.extend_from_slice(&[0x82, 15]);
    for (name, tile) in [
        ("a packed index", packed),
        ("a single pixel palette RLE index", single_pixel_index),
        ("a run palette RLE index", run_index),
    ] {
        let mut harness = DecodeHarness::new(4, 4);
        let payload = ZlibFixture::new().payload(&tile);
        let result = harness.decode(ENCODING_ZRLE, Rect::new(0, 0, 4, 4), &payload);
        assert!(
            matches!(result, Err(DecodeError::Corrupt(_))),
            "{name} outside the palette must be refused, got {result:?}"
        );
    }
}
