use crate::PixelConverter;
use crate::test_support::{bgr_little_endian, rgba_pixel};

#[test]
fn test_a_bgr_little_endian_format_swaps_red_and_blue() {
    let converter = PixelConverter::new(&bgr_little_endian());
    assert_eq!(
        converter.rgba(&[30, 20, 10, 0]),
        rgba_pixel(10, 20, 30),
        "blue comes first in the wire bytes"
    );
    assert_eq!(
        converter.compact_bytes_per_pixel(),
        3,
        "the colour bits fit the three low bytes"
    );
    assert_eq!(
        converter.compact_rgba(&[30, 20, 10]),
        rgba_pixel(10, 20, 30),
        "a ZRLE pixel keeps the wire order of the full pixel"
    );
    assert_eq!(
        converter.tight_bytes_per_pixel(),
        3,
        "a 24 bit Tight pixel is three bytes"
    );
    assert_eq!(
        converter.tight_rgba(&[10, 20, 30]),
        rgba_pixel(10, 20, 30),
        "a Tight pixel is red green blue whatever the shifts are"
    );
}
