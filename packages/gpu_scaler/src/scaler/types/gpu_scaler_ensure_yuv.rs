use super::GpuScaler;
use crate::{YuvShape, create_yuv_textures};

impl GpuScaler {
    pub(super) fn ensure_yuv(&mut self, device: &wgpu::Device, shape: YuvShape) {
        if self.yuv.as_ref().is_some_and(|yuv| yuv.shape == shape) {
            return;
        }
        self.yuv = Some(create_yuv_textures(
            device,
            &self.convert_layout,
            &self.convert_params,
            shape,
        ));
        log::debug!("[GPU] video planes recreated for {shape:?}");
    }
}
