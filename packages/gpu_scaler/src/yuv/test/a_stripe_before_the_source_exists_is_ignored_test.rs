use crate::test_support::{PLANES_AS_RGB, TEST_TARGET_FORMAT, TestGpu};
use crate::{GpuScaler, YuvLayout, YuvStripe};

#[test]
fn test_a_stripe_before_the_source_exists_is_ignored() {
    let Some(gpu) = TestGpu::new() else {
        return;
    };
    let mut scaler = GpuScaler::new(&gpu.device, TEST_TARGET_FORMAT);

    scaler.upload_yuv(
        &gpu.device,
        &gpu.queue,
        YuvStripe {
            y: 0,
            width: 2,
            height: 2,
            plane_width: 2,
            plane_height: 2,
            layout: YuvLayout::Planar444,
            colour: PLANES_AS_RGB,
            planes: &[0; 12],
        },
    );

    assert_eq!(
        scaler.source_size(),
        None,
        "a stripe must not create a source"
    );
}
