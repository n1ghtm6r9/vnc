use crate::PixelFormat;

#[test]
fn test_a_pixel_format_survives_a_round_trip() {
    let format = PixelFormat {
        bits_per_pixel: 16,
        depth: 16,
        big_endian: true,
        true_colour: true,
        red_max: 31,
        green_max: 63,
        blue_max: 31,
        red_shift: 11,
        green_shift: 5,
        blue_shift: 0,
    };
    let mut bytes = Vec::new();
    format.write(&mut bytes).unwrap();
    assert_eq!(bytes.len(), 16, "a pixel format is 13 bytes of values and 3 of padding");
    assert_eq!(PixelFormat::read(&mut &bytes[..]).unwrap(), format, "every field reads back unchanged");
}
