use crate::test_support::{
    ImageOptions, TEST_TARGET_FORMAT, TargetOptions, TestGpu, assert_bytes_close, pixel_at,
    render_image, uniform_image,
};
use crate::{ScaleFilter, Viewport};

#[test]
fn test_a_transparent_source_renders_opaque() {
    let Some(gpu) = TestGpu::new() else {
        return;
    };
    let source = uniform_image(8, 6, [90, 140, 200, 0]);
    for filter in [ScaleFilter::Smooth, ScaleFilter::Nearest] {
        let output = render_image(
            &gpu,
            ImageOptions {
                source_width: 8,
                source_height: 6,
                source: &source,
                target: TargetOptions {
                    format: TEST_TARGET_FORMAT,
                    width: 5,
                    height: 4,
                    viewport: Viewport::new(0, 0, 5, 4),
                    filter,
                    background: wgpu::Color::BLACK,
                },
            },
        );
        for y in 0..4 {
            for x in 0..5 {
                assert_bytes_close(
                    &pixel_at(&output, 5, x, y),
                    &[90, 140, 200, 255],
                    1,
                    &format!("the {filter:?} pixel at {x},{y} of a source without alpha"),
                );
            }
        }
    }
}
