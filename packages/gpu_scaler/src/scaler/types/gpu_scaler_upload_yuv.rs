use super::GpuScaler;
use crate::{
    ConvertParams, YuvShape, YuvStripe, begin_load_pass, draw_fullscreen_triangle, write_yuv_planes,
};

impl GpuScaler {
    pub fn upload_yuv(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        stripe: YuvStripe<'_>,
    ) {
        let Some((width, height)) = self.source_size() else {
            log::warn!("[GPU] ignoring a video stripe because the source has no size");
            return;
        };
        let within_limits =
            stripe.plane_width <= self.max_dimension && stripe.plane_height <= self.max_dimension;
        if !within_limits || !stripe.fits(width, height) {
            log::warn!(
                "[GPU] ignoring a {}x{} video stripe at row {} from {}x{} planes of {} bytes for the {width}x{height} source",
                stripe.width,
                stripe.height,
                stripe.y,
                stripe.plane_width,
                stripe.plane_height,
                stripe.planes.len()
            );
            return;
        }
        self.ensure_yuv(
            device,
            YuvShape {
                layout: stripe.layout,
                width: stripe.plane_width,
                height: stripe.plane_height,
            },
        );
        let (Some(source), Some(yuv)) = (&self.source, &self.yuv) else {
            return;
        };
        write_yuv_planes(queue, yuv, &stripe);
        queue.write_buffer(
            &self.convert_params,
            0,
            bytemuck::bytes_of(&ConvertParams::new(&stripe)),
        );
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("gpu scaler yuv conversion"),
        });
        {
            let mut pass = begin_load_pass(&mut encoder, &source.view);
            pass.set_scissor_rect(0, stripe.y, stripe.width, stripe.height);
            draw_fullscreen_triangle(&mut pass, &self.convert_pipeline, &yuv.bind_group);
        }
        queue.submit(Some(encoder.finish()));
    }
}
