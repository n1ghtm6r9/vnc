use super::Framebuffer;

impl Framebuffer {
    pub fn pixels(&self) -> &[u32] {
        &self.pixels
    }
}
