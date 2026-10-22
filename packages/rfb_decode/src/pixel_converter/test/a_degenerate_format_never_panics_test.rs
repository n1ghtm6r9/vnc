use rfb::PixelFormat;

use crate::PixelConverter;

#[test]
fn test_a_degenerate_format_never_panics() {
    let bytes = [0xFFu8; 16];
    for bits_per_pixel in [0u8, 3, 12, 24, 64, 255] {
        for big_endian in [false, true] {
            for true_colour in [false, true] {
                for (max, shift) in [(0u16, 0u8), (1, 255), (65535, 31), (65535, 200), (127, 24)] {
                    let format = PixelFormat {
                        bits_per_pixel,
                        depth: bits_per_pixel,
                        big_endian,
                        true_colour,
                        red_max: max,
                        green_max: max,
                        blue_max: max,
                        red_shift: shift,
                        green_shift: shift,
                        blue_shift: shift,
                    };
                    let mut converter = PixelConverter::new(&format);
                    converter.set_colour_map(65535, &[[1, 2, 3]; 3]);
                    for slice in [&bytes[..0], &bytes[..1], &bytes[..3], &bytes[..]] {
                        for pixel in [
                            converter.rgba(slice),
                            converter.compact_rgba(slice),
                            converter.tight_rgba(slice),
                        ] {
                            assert_eq!(
                                pixel >> 24,
                                255,
                                "every converted pixel is opaque for {format:?}"
                            );
                        }
                    }
                    let mut row = [0u32; 3];
                    converter.rgba_row(&bytes, &mut row);
                    converter.compact_rgba_row(&bytes, &mut row);
                    converter.tight_rgba_row(&bytes, &mut row);
                    assert!(
                        (1..=4).contains(&converter.bytes_per_pixel()),
                        "the pixel size stays within one to four bytes for {format:?}"
                    );
                }
            }
        }
    }
}
