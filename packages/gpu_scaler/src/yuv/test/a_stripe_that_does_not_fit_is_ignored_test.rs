use crate::test_support::{
    PLANES_AS_RGB, TEST_TARGET_FORMAT, TestGpu, assert_bytes_close, render_source, uniform_image,
};
use crate::{GpuScaler, YuvLayout, YuvStripe};

#[test]
fn test_a_stripe_that_does_not_fit_is_ignored() {
    let Some(gpu) = TestGpu::new() else {
        return;
    };
    let mut scaler = GpuScaler::new(&gpu.device, TEST_TARGET_FORMAT);
    scaler.resize_source(&gpu.device, 4, 4);
    let grey = vec![100; 48];
    let stripe = |y, width, height, planes| YuvStripe {
        y,
        width,
        height,
        plane_width: 4,
        plane_height: 4,
        layout: YuvLayout::Planar444,
        colour: PLANES_AS_RGB,
        planes,
    };
    scaler.upload_yuv(&gpu.device, &gpu.queue, stripe(0, 4, 4, &grey));

    let white = vec![255; 48];
    scaler.upload_yuv(&gpu.device, &gpu.queue, stripe(3, 4, 2, &white));
    scaler.upload_yuv(&gpu.device, &gpu.queue, stripe(u32::MAX, 4, 2, &white));
    scaler.upload_yuv(&gpu.device, &gpu.queue, stripe(0, 4, 4, &white[..47]));
    scaler.upload_yuv(&gpu.device, &gpu.queue, stripe(0, 5, 4, &white));
    scaler.upload_yuv(&gpu.device, &gpu.queue, stripe(0, 4, 0, &white));
    scaler.upload_yuv(
        &gpu.device,
        &gpu.queue,
        YuvStripe {
            plane_height: 2,
            planes: &white[..24],
            ..stripe(0, 4, 4, &white)
        },
    );

    assert_bytes_close(
        &render_source(&gpu, &mut scaler),
        &uniform_image(4, 4, [100, 100, 100, 255]),
        1,
        "the source after stripes that must have been ignored",
    );
}
