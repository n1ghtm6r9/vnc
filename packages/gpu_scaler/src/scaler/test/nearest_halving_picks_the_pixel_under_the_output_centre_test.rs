use crate::test_support::{
    ImageOptions, TEST_TARGET_FORMAT, TargetOptions, TestGpu, assert_bytes_close, pixel_at,
    random_image, render_image,
};
use crate::{ScaleFilter, Viewport};

#[test]
fn test_nearest_halving_picks_the_pixel_under_the_output_centre() {
    let Some(gpu) = TestGpu::new() else {
        return;
    };
    let source = random_image(6, 4, 41);
    let output = render_image(
        &gpu,
        ImageOptions {
            source_width: 6,
            source_height: 4,
            source: &source,
            target: TargetOptions {
                format: TEST_TARGET_FORMAT,
                width: 3,
                height: 2,
                viewport: Viewport::new(0, 0, 3, 2),
                filter: ScaleFilter::Nearest,
                background: wgpu::Color::BLACK,
            },
        },
    );
    for y in 0..2 {
        for x in 0..3 {
            assert_bytes_close(
                &pixel_at(&output, 3, x, y),
                &pixel_at(&source, 6, 2 * x + 1, 2 * y + 1),
                1,
                &format!("the pixel at {x},{y} of a nearest halving"),
            );
        }
    }
}
