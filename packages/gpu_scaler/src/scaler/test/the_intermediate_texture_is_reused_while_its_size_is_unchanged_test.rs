use crate::test_support::{TEST_TARGET_FORMAT, TargetOptions, TestGpu, render_scaler};
use crate::{GpuScaler, ScaleFilter, Viewport};

#[test]
fn test_the_intermediate_texture_is_reused_while_its_size_is_unchanged() {
    let Some(gpu) = TestGpu::new() else {
        return;
    };
    let mut scaler = GpuScaler::new(&gpu.device, TEST_TARGET_FORMAT);
    scaler.resize_source(&gpu.device, 8, 6);
    assert!(
        scaler.ensure_intermediate(&gpu.device, 16, 6),
        "the first request has no texture to reuse"
    );
    assert!(
        !scaler.ensure_intermediate(&gpu.device, 16, 6),
        "the same size must reuse the texture"
    );
    assert!(
        scaler.ensure_intermediate(&gpu.device, 17, 6),
        "a new width needs a new texture"
    );
    assert!(
        scaler.ensure_intermediate(&gpu.device, 17, 7),
        "a new height needs a new texture"
    );

    let mut scaler = GpuScaler::new(&gpu.device, TEST_TARGET_FORMAT);
    scaler.resize_source(&gpu.device, 8, 6);
    render_scaler(
        &gpu,
        &mut scaler,
        TargetOptions {
            format: TEST_TARGET_FORMAT,
            width: 20,
            height: 10,
            viewport: Viewport::new(-4, 2, 16, 8),
            filter: ScaleFilter::Smooth,
            background: wgpu::Color::BLACK,
        },
    );
    assert!(
        !scaler.ensure_intermediate(&gpu.device, 16, 6),
        "a render must size the intermediate texture from the viewport width and the source height"
    );
}
