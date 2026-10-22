use crate::test_support::{
    TEST_TARGET_FORMAT, TargetOptions, TestGpu, assert_bytes_close, pixel_at, random_image,
    render_scaler,
};
use crate::{GpuScaler, ScaleFilter, SourceRect, Viewport};

#[test]
fn test_a_one_pixel_wide_upload_lands_in_its_column() {
    let Some(gpu) = TestGpu::new() else {
        return;
    };
    let mut scaler = GpuScaler::new(&gpu.device, TEST_TARGET_FORMAT);
    scaler.resize_source(&gpu.device, 6, 9);
    let column = random_image(1, 7, 11);
    scaler.upload(&gpu.queue, SourceRect::new(3, 1, 1, 7), &column);
    let output = render_scaler(
        &gpu,
        &mut scaler,
        TargetOptions {
            format: TEST_TARGET_FORMAT,
            width: 6,
            height: 9,
            viewport: Viewport::new(0, 0, 6, 9),
            filter: ScaleFilter::Smooth,
            background: wgpu::Color::BLACK,
        },
    );
    for y in 0..9 {
        for x in 0..6 {
            let inside = x == 3 && (1..8).contains(&y);
            let expected = if inside {
                pixel_at(&column, 1, 0, y - 1)
            } else {
                [0, 0, 0, 255]
            };
            assert_bytes_close(
                &pixel_at(&output, 6, x, y),
                &expected,
                1,
                &format!("the pixel at {x},{y} (inside the uploaded column: {inside})"),
            );
        }
    }
}
