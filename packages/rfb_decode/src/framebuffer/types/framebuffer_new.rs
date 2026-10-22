use super::Framebuffer;
use crate::OPAQUE_BLACK;

impl Framebuffer {
    pub fn new(width: u16, height: u16) -> Framebuffer {
        let pixel_count = usize::from(width) * usize::from(height);
        Framebuffer {
            width,
            height,
            pixels: vec![OPAQUE_BLACK; pixel_count],
        }
    }
}
