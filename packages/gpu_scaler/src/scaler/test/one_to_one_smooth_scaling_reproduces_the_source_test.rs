use crate::test_support::{
    ImageOptions, TEST_TARGET_FORMAT, TargetOptions, TestGpu, assert_bytes_close, random_image,
    render_image,
};
use crate::{ScaleFilter, Viewport};

#[test]
fn test_one_to_one_smooth_scaling_reproduces_the_source() {
    let Some(gpu) = TestGpu::new() else {
        return;
    };
    let (width, height) = (67, 43);
    let source = random_image(width, height, 0x1234_5678);
    let output = render_image(
        &gpu,
        ImageOptions {
            source_width: width,
            source_height: height,
            source: &source,
            target: TargetOptions {
                format: TEST_TARGET_FORMAT,
                width,
                height,
                viewport: Viewport::new(0, 0, width, height),
                filter: ScaleFilter::Smooth,
                background: wgpu::Color::BLACK,
            },
        },
    );
    assert_bytes_close(
        &output,
        &source,
        1,
        "a 1:1 smooth render of a random pattern",
    );
}
