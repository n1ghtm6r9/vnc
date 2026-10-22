use crate::test_support::{
    ImageOptions, TEST_TARGET_FORMAT, TargetOptions, TestGpu, assert_bytes_close, render_image,
    uniform_image,
};
use crate::{ScaleFilter, Viewport};

#[test]
fn test_halving_a_uniform_colour_keeps_the_colour() {
    let Some(gpu) = TestGpu::new() else {
        return;
    };
    let colour = [200, 120, 40, 255];
    let source = uniform_image(64, 48, colour);
    let output = render_image(
        &gpu,
        ImageOptions {
            source_width: 64,
            source_height: 48,
            source: &source,
            target: TargetOptions {
                format: TEST_TARGET_FORMAT,
                width: 32,
                height: 24,
                viewport: Viewport::new(0, 0, 32, 24),
                filter: ScaleFilter::Smooth,
                background: wgpu::Color::BLACK,
            },
        },
    );
    assert_bytes_close(
        &output,
        &uniform_image(32, 24, colour),
        1,
        "a halved uniform colour",
    );
}
