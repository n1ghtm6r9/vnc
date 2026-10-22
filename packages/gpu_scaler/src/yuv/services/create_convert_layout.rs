use crate::{
    CHROMA_BLUE_BINDING, CHROMA_RED_BINDING, CONVERSION_BINDING, ConvertParams, LUMA_BINDING,
};

pub(crate) fn create_convert_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    let plane = |binding| wgpu::BindGroupLayoutEntry {
        binding,
        visibility: wgpu::ShaderStages::FRAGMENT,
        ty: wgpu::BindingType::Texture {
            sample_type: wgpu::TextureSampleType::Float { filterable: false },
            view_dimension: wgpu::TextureViewDimension::D2,
            multisampled: false,
        },
        count: None,
    };
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("gpu scaler yuv bind group layout"),
        entries: &[
            plane(LUMA_BINDING),
            plane(CHROMA_BLUE_BINDING),
            plane(CHROMA_RED_BINDING),
            wgpu::BindGroupLayoutEntry {
                binding: CONVERSION_BINDING,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: wgpu::BufferSize::new(size_of::<ConvertParams>() as u64),
                },
                count: None,
            },
        ],
    })
}
