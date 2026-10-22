use crate::test_support::{
    TEST_TARGET_FORMAT, TargetOptions, TestGpu, assert_bytes_close, random_image, render_scaler,
    uniform_image,
};
use crate::{GpuScaler, ScaleFilter, SourceRect, Viewport};

#[test]
fn test_an_upload_that_does_not_fit_is_ignored() {
    let Some(gpu) = TestGpu::new() else {
        return;
    };
    let mut scaler = GpuScaler::new(&gpu.device, TEST_TARGET_FORMAT);
    scaler.resize_source(&gpu.device, 6, 4);
    let source = random_image(6, 4, 17);
    scaler.upload(&gpu.queue, SourceRect::new(0, 0, 6, 4), &source);

    let white = uniform_image(4, 3, [255, 255, 255, 255]);
    scaler.upload(&gpu.queue, SourceRect::new(4, 2, 4, 3), &white);
    scaler.upload(&gpu.queue, SourceRect::new(u32::MAX, 0, 4, 3), &white);
    scaler.upload(&gpu.queue, SourceRect::new(0, 0, 2, 2), &white[..12]);
    scaler.upload(&gpu.queue, SourceRect::new(0, 0, 4, 3), &white[..44]);
    scaler.upload(&gpu.queue, SourceRect::new(0, 0, 0, 3), &[]);
    scaler.upload(&gpu.queue, SourceRect::new(0, 0, 4, 0), &white);

    let output = render_scaler(
        &gpu,
        &mut scaler,
        TargetOptions {
            format: TEST_TARGET_FORMAT,
            width: 6,
            height: 4,
            viewport: Viewport::new(0, 0, 6, 4),
            filter: ScaleFilter::Smooth,
            background: wgpu::Color::BLACK,
        },
    );
    assert_bytes_close(
        &output,
        &source,
        1,
        "a source after uploads that must have been ignored",
    );
}
