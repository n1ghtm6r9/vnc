use crate::{INPUT_BINDING, InputTexture, PARAMS_BINDING, TextureOptions};

pub(crate) fn create_input_texture(
    device: &wgpu::Device,
    layout: &wgpu::BindGroupLayout,
    params: &wgpu::Buffer,
    options: TextureOptions,
) -> InputTexture {
    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some(options.label),
        size: wgpu::Extent3d {
            width: options.width,
            height: options.height,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: options.format,
        usage: options.usage,
        view_formats: &[],
    });
    let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some(options.label),
        layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: INPUT_BINDING,
                resource: wgpu::BindingResource::TextureView(&view),
            },
            wgpu::BindGroupEntry {
                binding: PARAMS_BINDING,
                resource: params.as_entire_binding(),
            },
        ],
    });
    InputTexture {
        texture,
        view,
        bind_group,
        width: options.width,
        height: options.height,
    }
}
