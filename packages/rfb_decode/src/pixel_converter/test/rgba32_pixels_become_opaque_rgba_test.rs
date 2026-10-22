use rfb::PixelFormat;

use crate::PixelConverter;
use crate::test_support::rgba_pixel;

#[test]
fn test_rgba32_pixels_become_opaque_rgba() {
    let converter = PixelConverter::new(&PixelFormat::rgba32());
    assert_eq!(converter.bytes_per_pixel(), 4, "a full pixel is four bytes");
    assert_eq!(
        converter.compact_bytes_per_pixel(),
        3,
        "a ZRLE pixel drops the padding byte"
    );
    assert_eq!(
        converter.tight_bytes_per_pixel(),
        3,
        "a Tight pixel drops the padding byte"
    );
    assert_eq!(
        converter.rgba(&[10, 20, 30, 99]),
        rgba_pixel(10, 20, 30),
        "the padding byte never reaches the alpha channel"
    );
    assert_eq!(
        converter.compact_rgba(&[10, 20, 30]),
        rgba_pixel(10, 20, 30),
        "a ZRLE pixel reads as red green blue"
    );
    assert_eq!(
        converter.tight_rgba(&[10, 20, 30]),
        rgba_pixel(10, 20, 30),
        "a Tight pixel reads as red green blue"
    );
}
