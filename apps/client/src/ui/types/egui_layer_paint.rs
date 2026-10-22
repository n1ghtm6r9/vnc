use egui_wgpu::ScreenDescriptor;

use super::{EguiFrame, EguiLayer, EguiPaint};

impl EguiLayer {
    pub fn paint(&mut self, paint: EguiPaint, mut frame: EguiFrame) -> Vec<wgpu::CommandBuffer> {
        for (id, deltas) in frame.textures_delta.set.drain() {
            for delta in deltas {
                self.renderer.update_texture(paint.device, paint.queue, id, &delta);
            }
        }
        let screen = ScreenDescriptor {
            size_in_pixels: [paint.size.width, paint.size.height],
            pixels_per_point: frame.pixels_per_point,
        };
        let buffers = self.renderer.update_buffers(
            paint.device,
            paint.queue,
            paint.encoder,
            &frame.primitives,
            &screen,
        );
        let mut pass = paint
            .encoder
            .begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("egui"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: paint.target,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Store,
                    },
                })],
                ..Default::default()
            })
            .forget_lifetime();
        self.renderer.render(&mut pass, &frame.primitives, &screen);
        drop(pass);
        for id in frame.textures_delta.free.drain() {
            self.renderer.free_texture(&id);
        }
        buffers
    }
}
