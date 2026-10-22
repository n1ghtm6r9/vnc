use crate::{
    CHROMA_BLUE_BINDING, CHROMA_RED_BINDING, CONVERSION_BINDING, LUMA_BINDING, YuvShape,
    YuvTextures,
};

pub(crate) fn create_yuv_textures(
    device: &wgpu::Device,
    layout: &wgpu::BindGroupLayout,
    params: &wgpu::Buffer,
    shape: YuvShape,
) -> YuvTextures {
    let planes: Vec<wgpu::Texture> = shape
        .layout
        .plane_shapes(shape.width, shape.height)
        .into_iter()
        .map(|plane| {
            device.create_texture(&wgpu::TextureDescriptor {
                label: Some("gpu scaler yuv plane"),
                size: wgpu::Extent3d {
                    width: plane.width,
                    height: plane.height,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: plane.format,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                view_formats: &[],
            })
        })
        .collect();
    let views: Vec<wgpu::TextureView> = planes
        .iter()
        .map(|plane| plane.create_view(&wgpu::TextureViewDescriptor::default()))
        .collect();
    let view = |index: usize| wgpu::BindingResource::TextureView(&views[index]);
    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("gpu scaler yuv planes"),
        layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: LUMA_BINDING,
                resource: view(0),
            },
            wgpu::BindGroupEntry {
                binding: CHROMA_BLUE_BINDING,
                resource: view(1),
            },
            wgpu::BindGroupEntry {
                binding: CHROMA_RED_BINDING,
                resource: view(views.len() - 1),
            },
            wgpu::BindGroupEntry {
                binding: CONVERSION_BINDING,
                resource: params.as_entire_binding(),
            },
        ],
    });
    YuvTextures {
        shape,
        planes,
        bind_group,
    }
}
