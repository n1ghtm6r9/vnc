use super::render_scaler;
use crate::test_support::{TEST_TARGET_FORMAT, TargetOptions, TestGpu};
use crate::{GpuScaler, ScaleFilter, Viewport};

pub fn render_source(gpu: &TestGpu, scaler: &mut GpuScaler) -> Vec<u8> {
    let (width, height) = scaler
        .source_size()
        .expect("the scaler has a source to show");
    render_scaler(
        gpu,
        scaler,
        TargetOptions {
            format: TEST_TARGET_FORMAT,
            width,
            height,
            viewport: Viewport::new(0, 0, width, height),
            filter: ScaleFilter::Nearest,
            background: wgpu::Color::BLACK,
        },
    )
}
