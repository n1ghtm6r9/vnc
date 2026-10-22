use crate::test_support::{
    PLANES_AS_RGB, TEST_TARGET_FORMAT, TestGpu, assert_bytes_close, pixel_at, render_source,
};
use crate::{GpuScaler, YuvLayout, YuvStripe};

#[test]
fn test_only_the_visible_part_of_the_planes_is_shown() {
    let Some(gpu) = TestGpu::new() else {
        return;
    };
    let mut scaler = GpuScaler::new(&gpu.device, TEST_TARGET_FORMAT);
    scaler.resize_source(&gpu.device, 4, 4);
    let luma: Vec<u8> = (0..18).map(|i| 10 + i * 12).collect();
    let blue: Vec<u8> = (0..18).map(|i| 240 - i * 12).collect();
    let red: Vec<u8> = (0..18).map(|i| 3 + i * 13).collect();
    let planes = [luma.as_slice(), &blue, &red].concat();

    scaler.upload_yuv(
        &gpu.device,
        &gpu.queue,
        YuvStripe {
            y: 1,
            width: 4,
            height: 2,
            plane_width: 6,
            plane_height: 3,
            layout: YuvLayout::Planar444,
            colour: PLANES_AS_RGB,
            planes: &planes,
        },
    );

    let output = render_source(&gpu, &mut scaler);
    for y in 0..4 {
        for x in 0..4 {
            let expected = if (1..3).contains(&y) {
                let index = ((y - 1) * 6 + x) as usize;
                [red[index], luma[index], blue[index], 255]
            } else {
                [0, 0, 0, 255]
            };
            assert_bytes_close(
                &pixel_at(&output, 4, x, y),
                &expected,
                1,
                &format!("the pixel at {x},{y}, rows 1 and 2 from the top left of wider planes"),
            );
        }
    }
}
