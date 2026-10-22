use rfb::PixelFormat;

use crate::PixelConverter;
use crate::test_support::{colour_map_8_bit, rgba_pixel};

#[test]
fn test_cpixel_and_tpixel_keep_the_full_pixel_size_when_the_rules_do_not_apply() {
    let deep = PixelConverter::new(&PixelFormat {
        depth: 32,
        ..PixelFormat::rgba32()
    });
    assert_eq!(
        deep.compact_bytes_per_pixel(),
        4,
        "a depth above 24 keeps the four byte CPIXEL"
    );
    assert_eq!(
        deep.tight_bytes_per_pixel(),
        4,
        "a depth above 24 keeps the four byte TPIXEL"
    );
    assert_eq!(
        deep.compact_rgba(&[1, 2, 3, 4]),
        rgba_pixel(1, 2, 3),
        "a full size CPIXEL reads like a plain pixel"
    );

    let spread = PixelConverter::new(&PixelFormat {
        green_shift: 12,
        blue_shift: 24,
        ..PixelFormat::rgba32()
    });
    assert_eq!(
        spread.compact_bytes_per_pixel(),
        4,
        "colour bits in both the lowest and the highest byte need four bytes"
    );
    assert_eq!(
        spread.tight_bytes_per_pixel(),
        3,
        "a 24 bit format with full channels is three bytes in Tight"
    );

    let narrow = PixelConverter::new(&PixelFormat {
        red_max: 127,
        ..PixelFormat::rgba32()
    });
    assert_eq!(
        narrow.compact_bytes_per_pixel(),
        3,
        "the colour bits still fit the low bytes"
    );
    assert_eq!(
        narrow.tight_bytes_per_pixel(),
        4,
        "a channel maximum below 255 rules out the three byte TPIXEL"
    );

    let indexed = PixelConverter::new(&colour_map_8_bit());
    assert_eq!(
        indexed.compact_bytes_per_pixel(),
        1,
        "a colour map pixel keeps its size in ZRLE"
    );
    assert_eq!(
        indexed.tight_bytes_per_pixel(),
        1,
        "a colour map pixel keeps its size in Tight"
    );
}
