use crate::{YuvStripe, YuvTextures};

pub(crate) fn write_yuv_planes(
    queue: &wgpu::Queue,
    textures: &YuvTextures,
    stripe: &YuvStripe<'_>,
) {
    let whole = stripe
        .layout
        .plane_shapes(stripe.plane_width, stripe.plane_height);
    let shown = stripe.layout.plane_shapes(stripe.width, stripe.height);
    let mut offset = 0;
    for ((texture, plane), visible) in textures.planes.iter().zip(&whole).zip(&shown) {
        let end = offset + plane.byte_len();
        queue.write_texture(
            texture.as_image_copy(),
            &stripe.planes[offset..end],
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(plane.row_bytes()),
                rows_per_image: Some(plane.height),
            },
            wgpu::Extent3d {
                width: visible.width,
                height: visible.height,
                depth_or_array_layers: 1,
            },
        );
        offset = end;
    }
}
