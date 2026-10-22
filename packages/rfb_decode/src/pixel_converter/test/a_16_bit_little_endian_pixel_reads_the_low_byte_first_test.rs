use rfb::PixelFormat;

use crate::PixelConverter;
use crate::test_support::{rgb565_big_endian, rgba_pixel};

#[test]
fn test_a_16_bit_little_endian_pixel_reads_the_low_byte_first() {
    let format = PixelFormat {
        big_endian: false,
        ..rgb565_big_endian()
    };
    let converter = PixelConverter::new(&format);
    assert_eq!(
        converter.rgba(&[0x08, 0x84]),
        rgba_pixel(132, 130, 66),
        "the same value arrives with its bytes swapped"
    );
}
