use super::{GpuScaler, PassParams, ScalerFrame};
use crate::{VisibleRegion, begin_color_pass, draw_fullscreen_triangle};

impl GpuScaler {
    pub(super) fn draw_smooth(&mut self, frame: ScalerFrame<'_>, visible: VisibleRegion) {
        let Some((source_width, source_height)) = self.source_size() else {
            return;
        };
        let viewport = frame.viewport;
        self.ensure_intermediate(frame.device, viewport.width, source_height);
        let (Some(source), Some(intermediate)) = (&self.source, &self.intermediate) else {
            return;
        };
        let horizontal = PassParams::new(
            [0, 0],
            [source_width, source_height],
            [viewport.width, source_height],
            false,
        );
        let vertical = PassParams::new(
            [viewport.x, viewport.y],
            [viewport.width, source_height],
            [viewport.width, viewport.height],
            self.encode_srgb,
        );
        frame
            .queue
            .write_buffer(&self.source_params, 0, bytemuck::bytes_of(&horizontal));
        frame
            .queue
            .write_buffer(&self.intermediate_params, 0, bytemuck::bytes_of(&vertical));
        {
            let mut pass =
                begin_color_pass(frame.encoder, &intermediate.view, wgpu::Color::TRANSPARENT);
            draw_fullscreen_triangle(&mut pass, &self.horizontal_pipeline, &source.bind_group);
        }
        let mut pass = begin_color_pass(frame.encoder, frame.target, frame.background);
        pass.set_scissor_rect(visible.x, visible.y, visible.width, visible.height);
        draw_fullscreen_triangle(&mut pass, &self.vertical_pipeline, &intermediate.bind_group);
    }
}
