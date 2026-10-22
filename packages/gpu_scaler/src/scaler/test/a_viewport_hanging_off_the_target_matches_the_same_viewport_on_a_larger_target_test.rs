use crate::test_support::{
    ImageOptions, TEST_TARGET_FORMAT, TargetOptions, TestGpu, assert_bytes_close, pixel_at,
    random_image, render_image,
};
use crate::{ScaleFilter, Viewport};

#[test]
fn test_a_viewport_hanging_off_the_target_matches_the_same_viewport_on_a_larger_target() {
    let Some(gpu) = TestGpu::new() else {
        return;
    };
    let source = random_image(7, 5, 3);
    let render = |filter, width, height, viewport| {
        render_image(
            &gpu,
            ImageOptions {
                source_width: 7,
                source_height: 5,
                source: &source,
                target: TargetOptions {
                    format: TEST_TARGET_FORMAT,
                    width,
                    height,
                    viewport,
                    filter,
                    background: wgpu::Color::BLACK,
                },
            },
        )
    };
    for filter in [ScaleFilter::Smooth, ScaleFilter::Nearest] {
        let whole = render(filter, 16, 16, Viewport::new(3, 2, 12, 12));
        let whole_pixel =
            |local_x: u32, local_y: u32| pixel_at(&whole, 16, 3 + local_x, 2 + local_y);

        let before_the_edges = render(filter, 10, 10, Viewport::new(-3, -2, 12, 12));
        for y in 0..10 {
            for x in 0..10 {
                let (local_x, local_y) = (x + 3, y + 2);
                let expected = if local_x < 12 && local_y < 12 {
                    whole_pixel(local_x, local_y)
                } else {
                    [0, 0, 0, 255]
                };
                assert_bytes_close(
                    &pixel_at(&before_the_edges, 10, x, y),
                    &expected,
                    0,
                    &format!(
                        "the {filter:?} pixel at {x},{y} of a viewport hanging off the left and top edges"
                    ),
                );
            }
        }

        let past_the_edges = render(filter, 10, 9, Viewport::new(6, 5, 12, 12));
        for y in 0..9 {
            for x in 0..10 {
                let expected = if x >= 6 && y >= 5 {
                    whole_pixel(x - 6, y - 5)
                } else {
                    [0, 0, 0, 255]
                };
                assert_bytes_close(
                    &pixel_at(&past_the_edges, 10, x, y),
                    &expected,
                    0,
                    &format!(
                        "the {filter:?} pixel at {x},{y} of a viewport hanging off the right and bottom edges"
                    ),
                );
            }
        }
    }
}
