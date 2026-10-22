use crate::test_support::{
    ImageOptions, TEST_TARGET_FORMAT, TargetOptions, TestGpu, assert_bytes_close, pixel_at,
    render_image, uniform_image,
};
use crate::{ScaleFilter, Viewport};

#[test]
fn test_a_retina_sized_source_fits_a_window() {
    let Some(gpu) = TestGpu::new() else {
        return;
    };
    let source = uniform_image(3456, 2234, [30, 60, 90, 255]);
    let background = wgpu::Color {
        r: 0.0,
        g: 1.0,
        b: 0.0,
        a: 1.0,
    };
    for filter in [ScaleFilter::Smooth, ScaleFilter::Nearest] {
        let output = render_image(
            &gpu,
            ImageOptions {
                source_width: 3456,
                source_height: 2234,
                source: &source,
                target: TargetOptions {
                    format: TEST_TARGET_FORMAT,
                    width: 2560,
                    height: 1440,
                    viewport: Viewport::new(166, 0, 2228, 1440),
                    filter,
                    background,
                },
            },
        );
        for (x, y, expected) in [
            (166, 0, [30, 60, 90, 255]),
            (1280, 720, [30, 60, 90, 255]),
            (2393, 1439, [30, 60, 90, 255]),
            (165, 700, [0, 255, 0, 255]),
            (2394, 700, [0, 255, 0, 255]),
            (0, 0, [0, 255, 0, 255]),
            (2559, 1439, [0, 255, 0, 255]),
        ] {
            assert_bytes_close(
                &pixel_at(&output, 2560, x, y),
                &expected,
                1,
                &format!("the {filter:?} pixel at {x},{y} of a fitted retina screen"),
            );
        }
    }
}
