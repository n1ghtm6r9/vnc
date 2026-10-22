use crate::test_support::{
    ImageOptions, TEST_TARGET_FORMAT, TargetOptions, TestGpu, assert_bytes_close, random_image,
    reference_resample, render_image,
};
use crate::{ScaleFilter, Viewport};

#[test]
fn test_smooth_scaling_at_fractional_ratios_matches_a_cpu_reference() {
    let Some(gpu) = TestGpu::new() else {
        return;
    };
    let cases = [
        ((69, 45), (44, 29)),
        ((13, 9), (29, 20)),
        ((40, 40), (27, 61)),
        ((50, 3), (20, 8)),
    ];
    for (source_size, output_size) in cases {
        let source = random_image(source_size.0, source_size.1, 21);
        let output = render_image(
            &gpu,
            ImageOptions {
                source_width: source_size.0,
                source_height: source_size.1,
                source: &source,
                target: TargetOptions {
                    format: TEST_TARGET_FORMAT,
                    width: output_size.0,
                    height: output_size.1,
                    viewport: Viewport::new(0, 0, output_size.0, output_size.1),
                    filter: ScaleFilter::Smooth,
                    background: wgpu::Color::BLACK,
                },
            },
        );
        assert_bytes_close(
            &output,
            &reference_resample(&source, source_size, output_size),
            2,
            &format!(
                "a {}x{} source scaled to {}x{}",
                source_size.0, source_size.1, output_size.0, output_size.1
            ),
        );
    }
}
