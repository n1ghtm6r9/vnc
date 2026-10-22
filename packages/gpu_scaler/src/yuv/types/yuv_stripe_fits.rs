use super::{PlaneShape, YuvStripe};
use crate::SourceRect;

impl YuvStripe<'_> {
    pub(crate) fn fits(&self, source_width: u32, source_height: u32) -> bool {
        let rect = SourceRect::new(0, self.y, self.width, self.height);
        let plane_bytes: usize = self
            .layout
            .plane_shapes(self.plane_width, self.plane_height)
            .iter()
            .map(PlaneShape::byte_len)
            .sum();
        !rect.is_empty()
            && self.width <= self.plane_width
            && self.height <= self.plane_height
            && rect.fits_within(source_width, source_height)
            && self.planes.len() == plane_bytes
    }
}
