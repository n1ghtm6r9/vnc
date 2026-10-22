use crate::test_support::{TEST_TARGET_FORMAT, TestGpu, read_texture, uniform_image};
use crate::{GpuScaler, ScaleFilter, ScalerFrame, SourceRect, Viewport};

#[test]
fn test_a_frame_reporting_a_zero_sized_target_draws_nothing() {
    let Some(gpu) = TestGpu::new() else {
        return;
    };
    let mut scaler = GpuScaler::new(&gpu.device, TEST_TARGET_FORMAT);
    scaler.resize_source(&gpu.device, 4, 4);
    scaler.upload(
        &gpu.queue,
        SourceRect::new(0, 0, 4, 4),
        &uniform_image(4, 4, [255, 255, 255, 255]),
    );
    let target = gpu.device.create_texture(&wgpu::TextureDescriptor {
        label: Some("minimized window target"),
        size: wgpu::Extent3d {
            width: 4,
            height: 4,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: TEST_TARGET_FORMAT,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
        view_formats: &[],
    });
    let view = target.create_view(&wgpu::TextureViewDescriptor::default());
    for (target_width, target_height) in [(0, 4), (4, 0), (0, 0)] {
        for filter in [ScaleFilter::Smooth, ScaleFilter::Nearest] {
            let mut encoder = gpu
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
            scaler.render(ScalerFrame {
                device: &gpu.device,
                queue: &gpu.queue,
                encoder: &mut encoder,
                target: &view,
                target_width,
                target_height,
                viewport: Viewport::new(0, 0, 4, 4),
                filter,
                background: wgpu::Color::GREEN,
            });
            gpu.queue.submit(Some(encoder.finish()));
        }
    }
    let output = read_texture(&gpu, &target);
    assert!(
        output.iter().all(|&byte| byte == 0),
        "a frame that reports a zero-sized target leaves the target untouched"
    );
}
