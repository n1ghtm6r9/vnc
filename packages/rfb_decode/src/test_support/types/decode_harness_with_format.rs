use rfb::PixelFormat;

use super::DecodeHarness;
use crate::{DecoderState, Framebuffer, PixelConverter};

impl DecodeHarness {
    pub fn with_format(width: u16, height: u16, format: &PixelFormat) -> DecodeHarness {
        DecodeHarness {
            converter: PixelConverter::new(format),
            state: DecoderState::new(),
            framebuffer: Framebuffer::new(width, height),
        }
    }
}
