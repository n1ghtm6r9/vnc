use rfb::PixelFormat;

use crate::PixelConverter;
use crate::test_support::rgba_pixel;

#[test]
fn test_cpixel_is_three_bytes_when_the_colour_bits_fit_the_low_bytes() {
    let little_endian = PixelConverter::new(&PixelFormat::rgba32());
    assert_eq!(
        little_endian.compact_bytes_per_pixel(),
        3,
        "little endian colour bits in bytes zero to two fit three bytes"
    );
    assert_eq!(
        little_endian.compact_rgba(&[10, 20, 30]),
        rgba_pixel(10, 20, 30),
        "the first three wire bytes carry the colour"
    );

    let big_endian_format = PixelFormat {
        big_endian: true,
        red_shift: 16,
        green_shift: 8,
        blue_shift: 0,
        ..PixelFormat::rgba32()
    };
    let big_endian = PixelConverter::new(&big_endian_format);
    assert_eq!(
        big_endian.compact_bytes_per_pixel(),
        3,
        "big endian colour bits in the low bytes fit three bytes"
    );
    assert_eq!(
        big_endian.compact_rgba(&[10, 20, 30]),
        rgba_pixel(10, 20, 30),
        "the three least significant bytes are sent most significant first"
    );
}
