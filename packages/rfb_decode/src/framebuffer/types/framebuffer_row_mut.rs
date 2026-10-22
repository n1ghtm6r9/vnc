use super::Framebuffer;

impl Framebuffer {
    pub(crate) fn row_mut(&mut self, x: u16, y: u16, len: u16) -> &mut [u32] {
        let in_bounds = y < self.height && u32::from(x) + u32::from(len) <= u32::from(self.width);
        if !in_bounds {
            return Default::default();
        }
        let start = usize::from(y) * usize::from(self.width) + usize::from(x);
        self.pixels
            .get_mut(start..start + usize::from(len))
            .unwrap_or_default()
    }
}
