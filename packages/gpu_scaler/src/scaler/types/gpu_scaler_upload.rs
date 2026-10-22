use super::{GpuScaler, SourceRect};

impl GpuScaler {
    pub fn upload(&self, queue: &wgpu::Queue, rect: SourceRect, rgba: &[u8]) {
        let Some(source) = &self.source else {
            log::warn!("[GPU] ignoring an upload because the source has no size");
            return;
        };
        if rect.is_empty() {
            return;
        }
        if !rect.fits_within(source.width, source.height) {
            log::warn!(
                "[GPU] ignoring an upload of {rect:?} outside the {}x{} source",
                source.width,
                source.height
            );
            return;
        }
        if u64::try_from(rgba.len()).ok() != Some(rect.byte_len()) {
            log::warn!(
                "[GPU] ignoring an upload of {rect:?} with {} bytes instead of {}",
                rgba.len(),
                rect.byte_len()
            );
            return;
        }
        queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &source.texture,
                mip_level: 0,
                origin: wgpu::Origin3d {
                    x: rect.x,
                    y: rect.y,
                    z: 0,
                },
                aspect: wgpu::TextureAspect::All,
            },
            rgba,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(rect.width * 4),
                rows_per_image: Some(rect.height),
            },
            wgpu::Extent3d {
                width: rect.width,
                height: rect.height,
                depth_or_array_layers: 1,
            },
        );
        queue.submit([]);
    }
}
