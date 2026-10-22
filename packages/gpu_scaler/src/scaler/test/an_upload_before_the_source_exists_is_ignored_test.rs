use crate::test_support::{TEST_TARGET_FORMAT, TestGpu, uniform_image};
use crate::{GpuScaler, SourceRect};

#[test]
fn test_an_upload_before_the_source_exists_is_ignored() {
    let Some(gpu) = TestGpu::new() else {
        return;
    };
    let mut scaler = GpuScaler::new(&gpu.device, TEST_TARGET_FORMAT);
    let white = uniform_image(2, 2, [255, 255, 255, 255]);
    scaler.upload(&gpu.queue, SourceRect::new(0, 0, 2, 2), &white);
    assert_eq!(
        scaler.source_size(),
        None,
        "an upload must not create a source"
    );
    scaler.resize_source(&gpu.device, 2, 2);
    assert_eq!(
        scaler.source_size(),
        Some((2, 2)),
        "the source exists once it has a size"
    );
}
