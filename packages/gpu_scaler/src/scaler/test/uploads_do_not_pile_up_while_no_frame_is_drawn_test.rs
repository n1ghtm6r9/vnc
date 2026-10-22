use crate::test_support::{TEST_TARGET_FORMAT, TestGpu};
use crate::{GpuScaler, SourceRect};

#[test]
fn test_uploads_do_not_pile_up_while_no_frame_is_drawn() {
    let Some(gpu) = TestGpu::new() else {
        return;
    };
    let allocated = || {
        gpu.device
            .generate_allocator_report()
            .map(|report| report.total_allocated_bytes)
    };
    let mut scaler = GpuScaler::new(&gpu.device, TEST_TARGET_FORMAT);
    scaler.resize_source(&gpu.device, 1024, 1024);
    let Some(before) = allocated() else {
        println!("the gpu reports no allocations");
        return;
    };
    let picture = vec![128u8; 1024 * 1024 * 4];
    for _ in 0..32 {
        scaler.upload(&gpu.queue, SourceRect::new(0, 0, 1024, 1024), &picture);
    }
    gpu.device
        .poll(wgpu::PollType::wait_indefinitely())
        .expect("the gpu finishes its work");
    let grown = allocated().unwrap_or(before).saturating_sub(before);
    assert!(
        grown < picture.len() as u64,
        "uploads reach the gpu without waiting for a frame, yet 32 pictures left {grown} bytes behind"
    );
}
