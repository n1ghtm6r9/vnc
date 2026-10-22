use super::{Viewport, VisibleRegion};

impl Viewport {
    pub(crate) fn visible_within(
        &self,
        target_width: u32,
        target_height: u32,
    ) -> Option<VisibleRegion> {
        let left = i64::from(self.x).clamp(0, i64::from(target_width));
        let top = i64::from(self.y).clamp(0, i64::from(target_height));
        let right = (i64::from(self.x) + i64::from(self.width)).clamp(0, i64::from(target_width));
        let bottom =
            (i64::from(self.y) + i64::from(self.height)).clamp(0, i64::from(target_height));
        if right <= left || bottom <= top {
            return None;
        }
        Some(VisibleRegion {
            x: u32::try_from(left).ok()?,
            y: u32::try_from(top).ok()?,
            width: u32::try_from(right - left).ok()?,
            height: u32::try_from(bottom - top).ok()?,
        })
    }
}
