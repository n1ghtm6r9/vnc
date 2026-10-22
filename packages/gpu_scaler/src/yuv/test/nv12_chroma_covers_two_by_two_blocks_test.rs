use crate::test_support::{
    PLANES_AS_RGB, TEST_TARGET_FORMAT, TestGpu, assert_bytes_close, pixel_at, render_source,
};
use crate::{GpuScaler, YuvLayout, YuvStripe};

#[test]
fn test_nv12_chroma_covers_two_by_two_blocks() {
    let Some(gpu) = TestGpu::new() else {
        return;
    };
    let mut scaler = GpuScaler::new(&gpu.device, TEST_TARGET_FORMAT);
    scaler.resize_source(&gpu.device, 4, 4);
    let luma: Vec<u8> = (0..16).map(|i| 8 + i * 15).collect();
    let chroma: [u8; 8] = [20, 220, 60, 180, 100, 140, 160, 40];
    let planes = [luma.as_slice(), &chroma].concat();

    scaler.upload_yuv(
        &gpu.device,
        &gpu.queue,
        YuvStripe {
            y: 0,
            width: 4,
            height: 4,
            plane_width: 4,
            plane_height: 4,
            layout: YuvLayout::Nv12,
            colour: PLANES_AS_RGB,
            planes: &planes,
        },
    );

    let output = render_source(&gpu, &mut scaler);
    for y in 0..4 {
        for x in 0..4 {
            let pair = ((y / 2) * 4 + (x / 2) * 2) as usize;
            let expected = [
                chroma[pair + 1],
                luma[(y * 4 + x) as usize],
                chroma[pair],
                255,
            ];
            assert_bytes_close(
                &pixel_at(&output, 4, x, y),
                &expected,
                1,
                &format!("the pixel at {x},{y} with the chroma of its two by two block"),
            );
        }
    }
}
