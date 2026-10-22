use crate::test_support::{
    ImageOptions, TEST_TARGET_FORMAT, TargetOptions, TestGpu, assert_bytes_close, pixel_at,
    render_image, uniform_image,
};
use crate::{ScaleFilter, Viewport};

#[test]
fn test_pixels_outside_the_viewport_get_the_background() {
    let Some(gpu) = TestGpu::new() else {
        return;
    };
    let source = uniform_image(20, 20, [255, 255, 255, 255]);
    let output = render_image(
        &gpu,
        ImageOptions {
            source_width: 20,
            source_height: 20,
            source: &source,
            target: TargetOptions {
                format: TEST_TARGET_FORMAT,
                width: 32,
                height: 24,
                viewport: Viewport::new(8, 6, 16, 12),
                filter: ScaleFilter::Smooth,
                background: wgpu::Color {
                    r: 0.0,
                    g: 1.0,
                    b: 0.0,
                    a: 1.0,
                },
            },
        },
    );
    for y in 0..24 {
        for x in 0..32 {
            let inside = (8..24).contains(&x) && (6..18).contains(&y);
            let expected = if inside {
                [255, 255, 255, 255]
            } else {
                [0, 255, 0, 255]
            };
            assert_bytes_close(
                &pixel_at(&output, 32, x, y),
                &expected,
                1,
                &format!("the pixel at {x},{y} (inside the viewport: {inside})"),
            );
        }
    }
}
