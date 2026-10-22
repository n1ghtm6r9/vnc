use crate::test_support::{
    ImageOptions, TEST_TARGET_FORMAT, TargetOptions, TestGpu, assert_bytes_close, pixel_at,
    render_image, uniform_image,
};
use crate::{ScaleFilter, Viewport};

#[test]
fn test_a_viewport_without_visible_area_only_clears_the_target() {
    let Some(gpu) = TestGpu::new() else {
        return;
    };
    let source = uniform_image(6, 4, [255, 255, 255, 255]);
    let limit = gpu.device.limits().max_texture_dimension_2d;
    let viewports = [
        Viewport::new(3, 2, 0, 5),
        Viewport::new(3, 2, 5, 0),
        Viewport::new(20, 20, 8, 8),
        Viewport::new(-8, 0, 8, 8),
        Viewport::new(i32::MIN, i32::MIN, 3, 3),
        Viewport::new(i32::MAX, i32::MAX, 3, 3),
        Viewport::new(0, 0, limit + 1, 8),
        Viewport::new(0, 0, 8, limit + 1),
        Viewport::new(0, 0, u32::MAX, u32::MAX),
    ];
    for filter in [ScaleFilter::Smooth, ScaleFilter::Nearest] {
        for viewport in viewports {
            let output = render_image(
                &gpu,
                ImageOptions {
                    source_width: 6,
                    source_height: 4,
                    source: &source,
                    target: TargetOptions {
                        format: TEST_TARGET_FORMAT,
                        width: 8,
                        height: 8,
                        viewport,
                        filter,
                        background: wgpu::Color {
                            r: 0.0,
                            g: 1.0,
                            b: 0.0,
                            a: 1.0,
                        },
                    },
                },
            );
            for y in 0..8 {
                for x in 0..8 {
                    assert_bytes_close(
                        &pixel_at(&output, 8, x, y),
                        &[0, 255, 0, 255],
                        1,
                        &format!("the pixel at {x},{y} of a {filter:?} render with {viewport:?}"),
                    );
                }
            }
        }
    }
}
