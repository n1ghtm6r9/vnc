use crate::GpuScaler;
use crate::test_support::{TEST_TARGET_FORMAT, TestGpu};

#[test]
fn test_the_source_size_follows_resize_source() {
    let Some(gpu) = TestGpu::new() else {
        return;
    };
    let mut scaler = GpuScaler::new(&gpu.device, TEST_TARGET_FORMAT);
    assert_eq!(scaler.source_size(), None, "a new scaler has no source");
    scaler.resize_source(&gpu.device, 7, 5);
    assert_eq!(
        scaler.source_size(),
        Some((7, 5)),
        "the size of the source is reported"
    );
    scaler.resize_source(&gpu.device, 3, 9);
    assert_eq!(
        scaler.source_size(),
        Some((3, 9)),
        "a second resize replaces the size"
    );
    scaler.resize_source(&gpu.device, 0, 0);
    assert_eq!(scaler.source_size(), None, "an empty size drops the source");
    scaler.resize_source(&gpu.device, 4, 4);
    scaler.resize_source(&gpu.device, u32::MAX, 4);
    assert_eq!(
        scaler.source_size(),
        None,
        "a size the gpu cannot hold drops the source"
    );
}
