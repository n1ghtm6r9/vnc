use crate::test_support::{
    SAME_PRIMARIES, TEST_TARGET_FORMAT, TestGpu, assert_bytes_close, pixel_at, render_source,
};
use crate::{GpuScaler, YuvColour, YuvLayout, YuvStripe};

#[test]
fn test_the_offset_column_of_the_rows_shifts_the_codes() {
    let Some(gpu) = TestGpu::new() else {
        return;
    };
    let mut scaler = GpuScaler::new(&gpu.device, TEST_TARGET_FORMAT);
    scaler.resize_source(&gpu.device, 2, 1);
    let planes = [255, 100, 0, 0, 0, 0];

    scaler.upload_yuv(
        &gpu.device,
        &gpu.queue,
        YuvStripe {
            y: 0,
            width: 2,
            height: 1,
            plane_width: 2,
            plane_height: 1,
            layout: YuvLayout::Planar444,
            colour: YuvColour {
                rows: [
                    [1.0, 0.0, 0.0, -0.5],
                    [1.0, 0.0, 0.0, 0.0],
                    [0.0, 0.0, 0.0, 0.25],
                ],
                primaries: SAME_PRIMARIES,
            },
            planes: &planes,
        },
    );

    let output = render_source(&gpu, &mut scaler);
    assert_bytes_close(
        &pixel_at(&output, 2, 0, 0),
        &[128, 255, 64, 255],
        1,
        "full luma less half the code range is half red, and the offset alone gives a quarter blue",
    );
    assert_bytes_close(
        &pixel_at(&output, 2, 1, 0),
        &[0, 100, 64, 255],
        1,
        "luma 100 less half the code range clamps red to black",
    );
}
