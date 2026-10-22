use rfb::PixelFormat;

pub fn colour_map_8_bit() -> PixelFormat {
    PixelFormat {
        bits_per_pixel: 8,
        depth: 8,
        big_endian: false,
        true_colour: false,
        red_max: 0,
        green_max: 0,
        blue_max: 0,
        red_shift: 0,
        green_shift: 0,
        blue_shift: 0,
    }
}
