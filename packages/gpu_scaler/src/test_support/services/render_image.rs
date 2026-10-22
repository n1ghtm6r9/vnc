use super::render_scaler;
use crate::test_support::{ImageOptions, TestGpu};
use crate::{GpuScaler, SourceRect};

pub fn render_image(gpu: &TestGpu, options: ImageOptions<'_>) -> Vec<u8> {
    let mut scaler = GpuScaler::new(&gpu.device, options.target.format);
    scaler.resize_source(&gpu.device, options.source_width, options.source_height);
    scaler.upload(
        &gpu.queue,
        SourceRect::new(0, 0, options.source_width, options.source_height),
        options.source,
    );
    render_scaler(gpu, &mut scaler, options.target)
}
