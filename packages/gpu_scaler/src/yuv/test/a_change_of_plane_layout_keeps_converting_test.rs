use crate::test_support::{
    PLANES_AS_RGB, TEST_TARGET_FORMAT, TestGpu, assert_bytes_close, pixel_at, render_source,
};
use crate::{GpuScaler, YuvLayout, YuvStripe};

#[test]
fn test_a_change_of_plane_layout_keeps_converting() {
    let Some(gpu) = TestGpu::new() else {
        return;
    };
    let mut scaler = GpuScaler::new(&gpu.device, TEST_TARGET_FORMAT);
    scaler.resize_source(&gpu.device, 4, 4);
    let planar = [[50; 8], [60; 8], [70; 8]].concat();
    let paired = [vec![150; 8], [160, 170].repeat(2)].concat();
    let stripe = |y, layout, planes| YuvStripe {
        y,
        width: 4,
        height: 2,
        plane_width: 4,
        plane_height: 2,
        layout,
        colour: PLANES_AS_RGB,
        planes,
    };

    scaler.upload_yuv(
        &gpu.device,
        &gpu.queue,
        stripe(0, YuvLayout::Planar444, &planar),
    );
    scaler.upload_yuv(&gpu.device, &gpu.queue, stripe(2, YuvLayout::Nv12, &paired));

    let output = render_source(&gpu, &mut scaler);
    for y in 0..4 {
        for x in 0..4 {
            let expected = if y < 2 {
                [70, 50, 60, 255]
            } else {
                [170, 150, 160, 255]
            };
            assert_bytes_close(
                &pixel_at(&output, 4, x, y),
                &expected,
                1,
                &format!("the pixel at {x},{y}, a planar stripe above an interleaved one"),
            );
        }
    }
}
