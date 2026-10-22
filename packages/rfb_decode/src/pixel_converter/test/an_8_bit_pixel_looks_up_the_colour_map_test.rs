use crate::PixelConverter;
use crate::test_support::{colour_map_8_bit, rgba_pixel};

#[test]
fn test_an_8_bit_pixel_looks_up_the_colour_map() {
    let mut converter = PixelConverter::new(&colour_map_8_bit());
    converter.set_colour_map(1, &[[0xFFFF, 0, 0], [0, 0x8000, 0xFFFF]]);
    assert_eq!(converter.bytes_per_pixel(), 1, "an 8 bit pixel is one byte");
    assert_eq!(
        converter.rgba(&[1]),
        rgba_pixel(255, 0, 0),
        "entry one holds red"
    );
    assert_eq!(
        converter.rgba(&[2]),
        rgba_pixel(0, 128, 255),
        "entry two keeps the high byte of each 16 bit intensity"
    );
    assert_eq!(
        converter.rgba(&[0]),
        rgba_pixel(0, 0, 0),
        "an entry the server never set is opaque black"
    );
    assert_eq!(
        converter.rgba(&[200]),
        rgba_pixel(0, 0, 0),
        "an index past the table is opaque black"
    );
    converter.set_colour_map(0, &[[0, 0xFFFF, 0]]);
    assert_eq!(
        converter.rgba(&[0]),
        rgba_pixel(0, 255, 0),
        "a later update replaces the entry"
    );
    assert_eq!(
        converter.rgba(&[1]),
        rgba_pixel(255, 0, 0),
        "entries outside the update stay as they were"
    );
}
