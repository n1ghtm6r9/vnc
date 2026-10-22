use super::DecodeHarness;

impl DecodeHarness {
    pub fn pixel(&self, x: u16, y: u16) -> u32 {
        let index = usize::from(y) * usize::from(self.framebuffer.width()) + usize::from(x);
        self.framebuffer.pixels()[index]
    }
}
