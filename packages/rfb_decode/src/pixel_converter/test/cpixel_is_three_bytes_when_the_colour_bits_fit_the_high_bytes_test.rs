use rfb::PixelFormat;

use crate::PixelConverter;
use crate::test_support::rgba_pixel;

#[test]
fn test_cpixel_is_three_bytes_when_the_colour_bits_fit_the_high_bytes() {
    let little_endian_format = PixelFormat {
        red_shift: 8,
        green_shift: 16,
        blue_shift: 24,
        ..PixelFormat::rgba32()
    };
    let little_endian = PixelConverter::new(&little_endian_format);
    assert_eq!(
        little_endian.compact_bytes_per_pixel(),
        3,
        "little endian colour bits in bytes one to three fit three bytes"
    );
    assert_eq!(
        little_endian.compact_rgba(&[10, 20, 30]),
        rgba_pixel(10, 20, 30),
        "the three most significant bytes follow the little endian order"
    );
    assert_eq!(
        little_endian.rgba(&[0, 10, 20, 30]),
        rgba_pixel(10, 20, 30),
        "the full pixel agrees with its compact form"
    );

    let big_endian_format = PixelFormat {
        big_endian: true,
        red_shift: 24,
        green_shift: 16,
        blue_shift: 8,
        ..PixelFormat::rgba32()
    };
    let big_endian = PixelConverter::new(&big_endian_format);
    assert_eq!(
        big_endian.compact_bytes_per_pixel(),
        3,
        "big endian colour bits in the high bytes fit three bytes"
    );
    assert_eq!(
        big_endian.compact_rgba(&[10, 20, 30]),
        rgba_pixel(10, 20, 30),
        "the three most significant bytes come first on the wire"
    );
    assert_eq!(
        big_endian.rgba(&[10, 20, 30, 0]),
        rgba_pixel(10, 20, 30),
        "the full pixel agrees with its compact form"
    );
}
