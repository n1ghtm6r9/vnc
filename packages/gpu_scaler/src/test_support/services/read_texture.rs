use crate::test_support::TestGpu;

pub fn read_texture(gpu: &TestGpu, texture: &wgpu::Texture) -> Vec<u8> {
    let row_bytes = texture.width() as usize * 4;
    let alignment = wgpu::COPY_BYTES_PER_ROW_ALIGNMENT as usize;
    let padded_row_bytes = row_bytes.div_ceil(alignment) * alignment;
    let height = texture.height();
    let buffer = gpu.device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("test readback"),
        size: (padded_row_bytes * height as usize) as u64,
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });
    let mut encoder = gpu
        .device
        .create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("test readback"),
        });
    encoder.copy_texture_to_buffer(
        texture.as_image_copy(),
        wgpu::TexelCopyBufferInfo {
            buffer: &buffer,
            layout: wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(padded_row_bytes as u32),
                rows_per_image: Some(height),
            },
        },
        texture.size(),
    );
    gpu.queue.submit(Some(encoder.finish()));
    let slice = buffer.slice(..);
    slice.map_async(wgpu::MapMode::Read, |result| {
        result.expect("the readback buffer must map")
    });
    gpu.device
        .poll(wgpu::PollType::wait_indefinitely())
        .expect("the device must finish the readback");
    let mapped = slice
        .get_mapped_range()
        .expect("a mapped buffer has a range");
    let mut pixels = Vec::with_capacity(row_bytes * height as usize);
    for row in mapped.chunks_exact(padded_row_bytes) {
        pixels.extend_from_slice(&row[..row_bytes]);
    }
    drop(mapped);
    buffer.unmap();
    pixels
}
