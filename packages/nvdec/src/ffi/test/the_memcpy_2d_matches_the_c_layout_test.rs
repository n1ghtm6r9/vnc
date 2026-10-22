use std::mem::{offset_of, size_of};

use crate::CudaMemcpy2d;

#[test]
fn test_the_memcpy_2d_matches_the_c_layout() {
    let fields = [
        (
            "src_x_in_bytes",
            offset_of!(CudaMemcpy2d, src_x_in_bytes),
            0,
        ),
        ("src_y", offset_of!(CudaMemcpy2d, src_y), 8),
        (
            "src_memory_type",
            offset_of!(CudaMemcpy2d, src_memory_type),
            16,
        ),
        ("src_host", offset_of!(CudaMemcpy2d, src_host), 24),
        ("src_device", offset_of!(CudaMemcpy2d, src_device), 32),
        ("src_array", offset_of!(CudaMemcpy2d, src_array), 40),
        ("src_pitch", offset_of!(CudaMemcpy2d, src_pitch), 48),
        (
            "dst_x_in_bytes",
            offset_of!(CudaMemcpy2d, dst_x_in_bytes),
            56,
        ),
        ("dst_y", offset_of!(CudaMemcpy2d, dst_y), 64),
        (
            "dst_memory_type",
            offset_of!(CudaMemcpy2d, dst_memory_type),
            72,
        ),
        ("dst_host", offset_of!(CudaMemcpy2d, dst_host), 80),
        ("dst_device", offset_of!(CudaMemcpy2d, dst_device), 88),
        ("dst_array", offset_of!(CudaMemcpy2d, dst_array), 96),
        ("dst_pitch", offset_of!(CudaMemcpy2d, dst_pitch), 104),
        (
            "width_in_bytes",
            offset_of!(CudaMemcpy2d, width_in_bytes),
            112,
        ),
        ("height", offset_of!(CudaMemcpy2d, height), 120),
    ];

    for (name, actual, expected) in fields {
        assert_eq!(actual, expected, "{name} sits where CUDA_MEMCPY2D keeps it");
    }
    assert_eq!(
        size_of::<CudaMemcpy2d>(),
        128,
        "the copy is as big as CUDA_MEMCPY2D"
    );
}
