use crate::{CONVERT_ENTRY, PipelineOptions, SOURCE_FORMAT, YUV_WGSL, create_pipeline};

pub(crate) fn create_convert_pipeline(
    device: &wgpu::Device,
    layout: &wgpu::BindGroupLayout,
) -> wgpu::RenderPipeline {
    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("gpu scaler yuv pipeline layout"),
        bind_group_layouts: &[Some(layout)],
        immediate_size: 0,
    });
    let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("gpu scaler yuv"),
        source: wgpu::ShaderSource::Wgsl(YUV_WGSL.into()),
    });
    create_pipeline(
        device,
        PipelineOptions {
            layout: &pipeline_layout,
            module: &module,
            fragment_entry: CONVERT_ENTRY,
            format: SOURCE_FORMAT,
        },
    )
}
