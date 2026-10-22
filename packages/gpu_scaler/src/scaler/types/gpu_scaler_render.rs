use super::{GpuScaler, ScalerFrame};
use crate::{ScaleFilter, begin_color_pass};

impl GpuScaler {
    pub fn render(&mut self, frame: ScalerFrame<'_>) {
        if frame.target_width == 0 || frame.target_height == 0 {
            return;
        }
        let fits_the_device = frame.viewport.width <= self.max_dimension
            && frame.viewport.height <= self.max_dimension;
        let visible = frame
            .viewport
            .visible_within(frame.target_width, frame.target_height)
            .filter(|_| fits_the_device && self.source.is_some());
        let Some(visible) = visible else {
            drop(begin_color_pass(
                frame.encoder,
                frame.target,
                frame.background,
            ));
            return;
        };
        match frame.filter {
            ScaleFilter::Smooth => self.draw_smooth(frame, visible),
            ScaleFilter::Nearest => self.draw_nearest(frame, visible),
        }
    }
}
