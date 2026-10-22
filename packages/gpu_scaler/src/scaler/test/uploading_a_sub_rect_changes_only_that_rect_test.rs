use crate::test_support::{
    TEST_TARGET_FORMAT, TargetOptions, TestGpu, assert_bytes_close, pixel_at, random_image,
    render_scaler,
};
use crate::{GpuScaler, ScaleFilter, SourceRect, Viewport};

#[test]
fn test_uploading_a_sub_rect_changes_only_that_rect() {
    let Some(gpu) = TestGpu::new() else {
        return;
    };
    let mut scaler = GpuScaler::new(&gpu.device, TEST_TARGET_FORMAT);
    scaler.resize_source(&gpu.device, 12, 10);
    let patch = random_image(4, 3, 7);
    scaler.upload(&gpu.queue, SourceRect::new(5, 2, 4, 3), &patch);
    let output = render_scaler(
        &gpu,
        &mut scaler,
        TargetOptions {
            format: TEST_TARGET_FORMAT,
            width: 12,
            height: 10,
            viewport: Viewport::new(0, 0, 12, 10),
            filter: ScaleFilter::Smooth,
            background: wgpu::Color::BLACK,
        },
    );
    for y in 0..10 {
        for x in 0..12 {
            let inside = (5..9).contains(&x) && (2..5).contains(&y);
            let expected = if inside {
                pixel_at(&patch, 4, x - 5, y - 2)
            } else {
                [0, 0, 0, 255]
            };
            assert_bytes_close(
                &pixel_at(&output, 12, x, y),
                &expected,
                1,
                &format!("the pixel at {x},{y} (inside the uploaded rect: {inside})"),
            );
        }
    }
}
