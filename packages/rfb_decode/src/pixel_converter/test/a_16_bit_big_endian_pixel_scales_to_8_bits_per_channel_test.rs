use crate::PixelConverter;
use crate::test_support::{rgb565_big_endian, rgba_pixel};

#[test]
fn test_a_16_bit_big_endian_pixel_scales_to_8_bits_per_channel() {
    let converter = PixelConverter::new(&rgb565_big_endian());
    assert_eq!(
        converter.bytes_per_pixel(),
        2,
        "a 16 bit pixel is two bytes"
    );
    assert_eq!(
        converter.compact_bytes_per_pixel(),
        2,
        "a 16 bit ZRLE pixel keeps its size"
    );
    assert_eq!(
        converter.tight_bytes_per_pixel(),
        2,
        "a 16 bit Tight pixel keeps its size"
    );
    assert_eq!(
        converter.rgba(&[0x84, 0x08]),
        rgba_pixel(132, 130, 66),
        "the 5 6 5 fields scale to 8 bits with rounding"
    );
    assert_eq!(
        converter.rgba(&[0xFF, 0xFF]),
        rgba_pixel(255, 255, 255),
        "full intensity scales to 255"
    );
    assert_eq!(
        converter.rgba(&[0x00, 0x00]),
        rgba_pixel(0, 0, 0),
        "zero intensity stays zero"
    );
}
