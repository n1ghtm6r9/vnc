use crate::test_support::{
    ImageOptions, TEST_TARGET_FORMAT, TargetOptions, TestGpu, assert_bytes_close, pixel_at,
    random_image, render_image,
};
use crate::{ScaleFilter, Viewport};

#[test]
fn test_nearest_doubling_repeats_each_pixel() {
    let Some(gpu) = TestGpu::new() else {
        return;
    };
    let source = random_image(5, 3, 99);
    let output = render_image(
        &gpu,
        ImageOptions {
            source_width: 5,
            source_height: 3,
            source: &source,
            target: TargetOptions {
                format: TEST_TARGET_FORMAT,
                width: 14,
                height: 9,
                viewport: Viewport::new(2, 1, 10, 6),
                filter: ScaleFilter::Nearest,
                background: wgpu::Color::BLACK,
            },
        },
    );
    for y in 0..9 {
        for x in 0..14 {
            let inside = (2..12).contains(&x) && (1..7).contains(&y);
            let expected = if inside {
                pixel_at(&source, 5, (x - 2) / 2, (y - 1) / 2)
            } else {
                [0, 0, 0, 255]
            };
            assert_bytes_close(
                &pixel_at(&output, 14, x, y),
                &expected,
                1,
                &format!("the pixel at {x},{y} of a nearest doubling"),
            );
        }
    }
}
