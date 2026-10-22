use crate::test_support::{
    PLANES_AS_RGB, TEST_TARGET_FORMAT, TestGpu, assert_bytes_close, pixel_at, render_source,
};
use crate::{GpuScaler, YuvLayout, YuvStripe};

#[test]
fn test_a_444_stripe_lands_in_its_rows_of_the_source() {
    let Some(gpu) = TestGpu::new() else {
        return;
    };
    let mut scaler = GpuScaler::new(&gpu.device, TEST_TARGET_FORMAT);
    scaler.resize_source(&gpu.device, 4, 4);
    let luma: Vec<u8> = (0..8).map(|i| 10 + i * 20).collect();
    let blue: Vec<u8> = (0..8).map(|i| 200 - i * 20).collect();
    let red: Vec<u8> = (0..8).map(|i| 5 + i * 30).collect();
    let planes = [luma.as_slice(), &blue, &red].concat();

    scaler.upload_yuv(
        &gpu.device,
        &gpu.queue,
        YuvStripe {
            y: 2,
            width: 4,
            height: 2,
            plane_width: 4,
            plane_height: 2,
            layout: YuvLayout::Planar444,
            colour: PLANES_AS_RGB,
            planes: &planes,
        },
    );

    let output = render_source(&gpu, &mut scaler);
    for y in 0..4 {
        for x in 0..4 {
            let expected = if y < 2 {
                [0, 0, 0, 255]
            } else {
                let index = ((y - 2) * 4 + x) as usize;
                [red[index], luma[index], blue[index], 255]
            };
            assert_bytes_close(
                &pixel_at(&output, 4, x, y),
                &expected,
                1,
                &format!("the source pixel at {x},{y}, below row 2 taken from the stripe"),
            );
        }
    }
}
