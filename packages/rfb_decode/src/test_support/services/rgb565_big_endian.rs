use rfb::PixelFormat;

pub fn rgb565_big_endian() -> PixelFormat {
    PixelFormat {
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
    }
}
