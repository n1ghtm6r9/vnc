use super::{GpuScaler, TextureOptions};
use crate::{SOURCE_FORMAT, create_input_texture};

impl GpuScaler {
    pub fn resize_source(&mut self, device: &wgpu::Device, width: u32, height: u32) {
        if width == 0 || height == 0 || width > self.max_dimension || height > self.max_dimension {
            log::warn!("[GPU] cannot hold a {width}x{height} source, the source is dropped");
            self.source = None;
            return;
        }
        self.source = Some(create_input_texture(
            device,
            &self.layout,
            &self.source_params,
            TextureOptions {
                label: "gpu scaler source",
                width,
                height,
                format: SOURCE_FORMAT,
                usage: wgpu::TextureUsages::TEXTURE_BINDING
                    | wgpu::TextureUsages::COPY_DST
                    | wgpu::TextureUsages::RENDER_ATTACHMENT,
            },
        ));
        log::debug!("[GPU] source resized to {width}x{height}");
    }
}
