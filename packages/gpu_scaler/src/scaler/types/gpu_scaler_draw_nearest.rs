use super::{GpuScaler, PassParams, ScalerFrame};
use crate::{VisibleRegion, begin_color_pass, draw_fullscreen_triangle};

impl GpuScaler {
    pub(super) fn draw_nearest(&self, frame: ScalerFrame<'_>, visible: VisibleRegion) {
        let Some(source) = &self.source else {
            return;
        };
        let viewport = frame.viewport;
        let params = PassParams::new(
            [viewport.x, viewport.y],
            [source.width, source.height],
            [viewport.width, viewport.height],
            self.encode_srgb,
        );
        frame
            .queue
            .write_buffer(&self.source_params, 0, bytemuck::bytes_of(&params));
        let mut pass = begin_color_pass(frame.encoder, frame.target, frame.background);
        pass.set_scissor_rect(visible.x, visible.y, visible.width, visible.height);
        draw_fullscreen_triangle(&mut pass, &self.nearest_pipeline, &source.bind_group);
    }
}
