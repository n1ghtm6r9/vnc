use super::PlaneShape;

impl PlaneShape {
    pub(crate) fn byte_len(&self) -> usize {
        self.width as usize * self.texel_bytes as usize * self.height as usize
    }
}
