use crate::test_support::{
    ImageOptions, TEST_TARGET_FORMAT, TargetOptions, TestGpu, assert_bytes_close, pixel_at,
    random_image, render_image,
};
use crate::{ScaleFilter, Viewport};

#[test]
fn test_a_panned_one_to_one_screen_shows_the_matching_part_of_the_source() {
    let Some(gpu) = TestGpu::new() else {
        return;
    };
    let source = random_image(40, 30, 13);
    for filter in [ScaleFilter::Smooth, ScaleFilter::Nearest] {
        let output = render_image(
            &gpu,
            ImageOptions {
                source_width: 40,
                source_height: 30,
                source: &source,
                target: TargetOptions {
                    format: TEST_TARGET_FORMAT,
                    width: 24,
                    height: 16,
                    viewport: Viewport::new(-7, -5, 40, 30),
                    filter,
                    background: wgpu::Color::BLACK,
                },
            },
        );
        for y in 0..16 {
            for x in 0..24 {
                assert_bytes_close(
                    &pixel_at(&output, 24, x, y),
                    &pixel_at(&source, 40, x + 7, y + 5),
                    1,
                    &format!("the {filter:?} pixel at {x},{y} of a panned screen"),
                );
            }
        }
    }
}
