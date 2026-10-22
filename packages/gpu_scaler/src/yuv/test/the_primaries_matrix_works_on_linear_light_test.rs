use crate::test_support::{
    PLANES_AS_RGB, TEST_TARGET_FORMAT, TestGpu, assert_bytes_close, linear_to_srgb, pixel_at,
    render_source, srgb_to_linear,
};
use crate::{GpuScaler, YuvColour, YuvLayout, YuvStripe};

#[test]
fn test_the_primaries_matrix_works_on_linear_light() {
    let Some(gpu) = TestGpu::new() else {
        return;
    };
    let mut scaler = GpuScaler::new(&gpu.device, TEST_TARGET_FORMAT);
    scaler.resize_source(&gpu.device, 2, 1);
    let red_codes = [255, 128];
    let planes = [[255, 255], [255, 255], red_codes].concat();
    let half_red = YuvColour {
        rows: PLANES_AS_RGB.rows,
        primaries: [[0.5, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
    };

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
            colour: half_red,
            planes: &planes,
        },
    );

    let output = render_source(&gpu, &mut scaler);
    for (x, code) in red_codes.into_iter().enumerate() {
        let expected = [linear_to_srgb(0.5 * srgb_to_linear(code)), 255, 255, 255];
        assert_bytes_close(
            &pixel_at(&output, 2, x as u32, 0),
            &expected,
            1,
            &format!("red code {code} after a matrix that halves the red light, not the red code"),
        );
    }
}
