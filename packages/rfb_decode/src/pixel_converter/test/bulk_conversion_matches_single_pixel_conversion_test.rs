use rfb::PixelFormat;

use crate::PixelConverter;
use crate::test_support::{bgr_little_endian, colour_map_8_bit, rgb565_big_endian};

#[test]
fn test_bulk_conversion_matches_single_pixel_conversion() {
    let formats = [
        PixelFormat::rgba32(),
        rgb565_big_endian(),
        bgr_little_endian(),
        colour_map_8_bit(),
        PixelFormat {
            depth: 32,
            ..PixelFormat::rgba32()
        },
        PixelFormat {
            red_shift: 8,
            green_shift: 16,
            blue_shift: 24,
            ..PixelFormat::rgba32()
        },
        PixelFormat {
            big_endian: true,
            red_shift: 24,
            green_shift: 16,
            blue_shift: 8,
            ..PixelFormat::rgba32()
        },
    ];
    let source: Vec<u8> = (0..120u32).map(|index| (index * 37 + 11) as u8).collect();
    let colours: Vec<[u16; 3]> = (0..256u16)
        .map(|index| [index * 251, index * 241, index * 239])
        .collect();
    let count = 20;
    for format in formats {
        let mut converter = PixelConverter::new(&format);
        converter.set_colour_map(0, &colours);

        let mut row = vec![0u32; count];
        converter.rgba_row(&source, &mut row);
        for (index, pixel) in source
            .chunks_exact(converter.bytes_per_pixel())
            .take(count)
            .enumerate()
        {
            assert_eq!(
                row[index],
                converter.rgba(pixel),
                "full pixel {index} converts the same in bulk for {format:?}"
            );
        }
        assert!(
            row.iter().any(|&pixel| pixel != row[0]),
            "the converted row holds more than one colour for {format:?}"
        );

        let mut row = vec![0u32; count];
        converter.compact_rgba_row(&source, &mut row);
        for (index, pixel) in source
            .chunks_exact(converter.compact_bytes_per_pixel())
            .take(count)
            .enumerate()
        {
            assert_eq!(
                row[index],
                converter.compact_rgba(pixel),
                "compact pixel {index} converts the same in bulk for {format:?}"
            );
        }

        let mut row = vec![0u32; count];
        converter.tight_rgba_row(&source, &mut row);
        for (index, pixel) in source
            .chunks_exact(converter.tight_bytes_per_pixel())
            .take(count)
            .enumerate()
        {
            assert_eq!(
                row[index],
                converter.tight_rgba(pixel),
                "tight pixel {index} converts the same in bulk for {format:?}"
            );
        }
    }
}
