use rfb::PixelFormat;

pub fn bgr_little_endian() -> PixelFormat {
    PixelFormat {
        red_shift: 16,
        green_shift: 8,
        blue_shift: 0,
        ..PixelFormat::rgba32()
    }
}
