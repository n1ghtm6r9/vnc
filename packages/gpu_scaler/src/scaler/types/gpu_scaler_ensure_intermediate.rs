use super::{GpuScaler, TextureOptions};
use crate::{INTERMEDIATE_FORMAT, create_input_texture};

impl GpuScaler {
    pub(crate) fn ensure_intermediate(
        &mut self,
        device: &wgpu::Device,
        width: u32,
        height: u32,
    ) -> bool {
        let reusable = self.intermediate.as_ref().is_some_and(|intermediate| {
            intermediate.width == width && intermediate.height == height
        });
        if reusable {
            return false;
        }
        self.intermediate = Some(create_input_texture(
            device,
            &self.layout,
            &self.intermediate_params,
            TextureOptions {
                label: "gpu scaler intermediate",
                width,
                height,
                format: INTERMEDIATE_FORMAT,
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                    | wgpu::TextureUsages::TEXTURE_BINDING,
            },
        ));
        log::debug!("[GPU] intermediate texture recreated at {width}x{height}");
        true
    }
}
