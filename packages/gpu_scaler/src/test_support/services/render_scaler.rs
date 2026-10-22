use super::read_texture;
use crate::test_support::{TargetOptions, TestGpu};
use crate::{GpuScaler, ScalerFrame};

pub fn render_scaler(gpu: &TestGpu, scaler: &mut GpuScaler, options: TargetOptions) -> Vec<u8> {
    let target = gpu.device.create_texture(&wgpu::TextureDescriptor {
        label: Some("test target"),
        size: wgpu::Extent3d {
            width: options.width,
            height: options.height,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: options.format,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
        view_formats: &[],
    });
    let view = target.create_view(&wgpu::TextureViewDescriptor::default());
    let mut encoder = gpu
        .device
        .create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("test frame"),
        });
    scaler.render(ScalerFrame {
        device: &gpu.device,
        queue: &gpu.queue,
        encoder: &mut encoder,
        target: &view,
        target_width: options.width,
        target_height: options.height,
        viewport: options.viewport,
        filter: options.filter,
        background: options.background,
    });
    gpu.queue.submit(Some(encoder.finish()));
    read_texture(gpu, &target)
}
