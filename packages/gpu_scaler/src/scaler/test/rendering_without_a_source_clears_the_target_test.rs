use crate::test_support::{
    TEST_TARGET_FORMAT, TargetOptions, TestGpu, assert_bytes_close, pixel_at, render_scaler,
};
use crate::{GpuScaler, ScaleFilter, Viewport};

#[test]
fn test_rendering_without_a_source_clears_the_target() {
    let Some(gpu) = TestGpu::new() else {
        return;
    };
    let mut scaler = GpuScaler::new(&gpu.device, TEST_TARGET_FORMAT);
    for filter in [ScaleFilter::Smooth, ScaleFilter::Nearest] {
        let output = render_scaler(
            &gpu,
            &mut scaler,
            TargetOptions {
                format: TEST_TARGET_FORMAT,
                width: 8,
                height: 6,
                viewport: Viewport::new(0, 0, 8, 6),
                filter,
                background: wgpu::Color {
                    r: 0.0,
                    g: 1.0,
                    b: 0.0,
                    a: 1.0,
                },
            },
        );
        for y in 0..6 {
            for x in 0..8 {
                assert_bytes_close(
                    &pixel_at(&output, 8, x, y),
                    &[0, 255, 0, 255],
                    1,
                    &format!("the pixel at {x},{y} of a {filter:?} render without a source"),
                );
            }
        }
    }
}
