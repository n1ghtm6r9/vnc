use std::fmt;

#[derive(Clone)]
pub struct Framebuffer {
    pub(super) width: u16,
    pub(super) height: u16,
    pub(super) pixels: Vec<u32>,
}

impl fmt::Debug for Framebuffer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Framebuffer")
            .field("width", &self.width)
            .field("height", &self.height)
            .finish_non_exhaustive()
    }
}
