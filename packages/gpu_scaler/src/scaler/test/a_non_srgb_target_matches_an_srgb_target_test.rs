use crate::test_support::{
    ImageOptions, TargetOptions, TestGpu, assert_bytes_close, random_image, render_image,
};
use crate::{ScaleFilter, Viewport};

#[test]
fn test_a_non_srgb_target_matches_an_srgb_target() {
    let Some(gpu) = TestGpu::new() else {
        return;
    };
    let source = random_image(61, 47, 5);
    for filter in [ScaleFilter::Smooth, ScaleFilter::Nearest] {
        let render = |format| {
            render_image(
                &gpu,
                ImageOptions {
                    source_width: 61,
                    source_height: 47,
                    source: &source,
                    target: TargetOptions {
                        format,
                        width: 40,
                        height: 32,
                        viewport: Viewport::new(2, 1, 37, 29),
                        filter,
                        background: wgpu::Color::BLACK,
                    },
                },
            )
        };
        let srgb = render(wgpu::TextureFormat::Bgra8UnormSrgb);
        let plain = render(wgpu::TextureFormat::Bgra8Unorm);
        assert!(
            srgb.iter().any(|byte| *byte > 32),
            "the {filter:?} render must contain the image and not only the background"
        );
        assert_bytes_close(
            &plain,
            &srgb,
            1,
            &format!("a {filter:?} render to a non-sRGB target"),
        );
    }
}
