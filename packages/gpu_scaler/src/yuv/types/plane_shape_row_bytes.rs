use super::PlaneShape;

impl PlaneShape {
    pub(crate) fn row_bytes(&self) -> u32 {
        self.width * self.texel_bytes
    }
}
