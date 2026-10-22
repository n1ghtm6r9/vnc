use crate::test_support::{
    TEST_TARGET_FORMAT, TargetOptions, TestGpu, assert_bytes_close, pixel_at, render_scaler,
    uniform_image,
};
use crate::{GpuScaler, ScaleFilter, SourceRect, Viewport};

#[test]
fn test_resizing_the_source_clears_it_to_black() {
    let Some(gpu) = TestGpu::new() else {
        return;
    };
    let mut scaler = GpuScaler::new(&gpu.device, TEST_TARGET_FORMAT);
    let options = || TargetOptions {
        format: TEST_TARGET_FORMAT,
        width: 8,
        height: 6,
        viewport: Viewport::new(0, 0, 8, 6),
        filter: ScaleFilter::Smooth,
        background: wgpu::Color {
            r: 0.0,
            g: 1.0,
            b: 0.0,
            a: 1.0,
        },
    };
    scaler.resize_source(&gpu.device, 8, 6);
    scaler.upload(
        &gpu.queue,
        SourceRect::new(0, 0, 8, 6),
        &uniform_image(8, 6, [255, 255, 255, 255]),
    );
    let before = render_scaler(&gpu, &mut scaler, options());
    assert_bytes_close(
        &pixel_at(&before, 8, 4, 3),
        &[255, 255, 255, 255],
        1,
        "the uploaded white source before the resize",
    );

    scaler.resize_source(&gpu.device, 8, 6);
    let after = render_scaler(&gpu, &mut scaler, options());
    for y in 0..6 {
        for x in 0..8 {
            assert_bytes_close(
                &pixel_at(&after, 8, x, y),
                &[0, 0, 0, 255],
                1,
                &format!("the pixel at {x},{y} after the source was resized"),
            );
        }
    }
}
