use crate::test_support::{
    ImageOptions, TEST_TARGET_FORMAT, TargetOptions, TestGpu, assert_bytes_close,
    checkerboard_image, pixel_at, render_image,
};
use crate::{ScaleFilter, Viewport};

#[test]
fn test_halving_a_black_and_white_checkerboard_gives_linear_grey() {
    let Some(gpu) = TestGpu::new() else {
        return;
    };
    let source = checkerboard_image(64, 64);
    let output = render_image(
        &gpu,
        ImageOptions {
            source_width: 64,
            source_height: 64,
            source: &source,
            target: TargetOptions {
                format: TEST_TARGET_FORMAT,
                width: 32,
                height: 32,
                viewport: Viewport::new(0, 0, 32, 32),
                filter: ScaleFilter::Smooth,
                background: wgpu::Color::BLACK,
            },
        },
    );
    for y in 3..29 {
        for x in 3..29 {
            assert_bytes_close(
                &pixel_at(&output, 32, x, y),
                &[188, 188, 188, 255],
                2,
                &format!("the pixel at {x},{y} of a halved checkerboard"),
            );
        }
    }
}
