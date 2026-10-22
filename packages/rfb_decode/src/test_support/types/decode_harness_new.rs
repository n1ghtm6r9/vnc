use rfb::PixelFormat;

use super::DecodeHarness;

impl DecodeHarness {
    pub fn new(width: u16, height: u16) -> DecodeHarness {
        DecodeHarness::with_format(width, height, &PixelFormat::rgba32())
    }
}
