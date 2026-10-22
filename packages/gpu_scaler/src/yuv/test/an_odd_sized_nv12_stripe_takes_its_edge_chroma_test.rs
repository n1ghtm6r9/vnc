use crate::test_support::{
    PLANES_AS_RGB, TEST_TARGET_FORMAT, TestGpu, assert_bytes_close, pixel_at, render_source,
};
use crate::{GpuScaler, YuvLayout, YuvStripe};

#[test]
fn test_an_odd_sized_nv12_stripe_takes_its_edge_chroma() {
    let Some(gpu) = TestGpu::new() else {
        return;
    };
    let mut scaler = GpuScaler::new(&gpu.device, TEST_TARGET_FORMAT);
    scaler.resize_source(&gpu.device, 3, 3);
    let luma: Vec<u8> = (0..9).map(|i| 30 + i * 20).collect();
    let chroma: [u8; 8] = [50, 200, 90, 160, 130, 120, 170, 80];
    let planes = [luma.as_slice(), &chroma].concat();

    scaler.upload_yuv(
        &gpu.device,
        &gpu.queue,
        YuvStripe {
            y: 0,
            width: 3,
            height: 3,
            plane_width: 3,
            plane_height: 3,
            layout: YuvLayout::Nv12,
            colour: PLANES_AS_RGB,
            planes: &planes,
        },
    );

    let output = render_source(&gpu, &mut scaler);
    for y in 0..3 {
        for x in 0..3 {
            let pair = ((y / 2) * 4 + (x / 2) * 2) as usize;
            let expected = [
                chroma[pair + 1],
                luma[(y * 3 + x) as usize],
                chroma[pair],
                255,
            ];
            assert_bytes_close(
                &pixel_at(&output, 3, x, y),
                &expected,
                1,
                &format!("the pixel at {x},{y}, the last row and column with the edge chroma"),
            );
        }
    }
}
