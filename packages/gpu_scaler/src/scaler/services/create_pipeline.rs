use crate::{PipelineOptions, VERTEX_ENTRY};

pub(crate) fn create_pipeline(
    device: &wgpu::Device,
    options: PipelineOptions<'_>,
) -> wgpu::RenderPipeline {
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some(options.fragment_entry),
        layout: Some(options.layout),
        vertex: wgpu::VertexState {
            module: options.module,
            entry_point: Some(VERTEX_ENTRY),
            compilation_options: wgpu::PipelineCompilationOptions::default(),
            buffers: &[],
        },
        primitive: wgpu::PrimitiveState::default(),
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        fragment: Some(wgpu::FragmentState {
            module: options.module,
            entry_point: Some(options.fragment_entry),
            compilation_options: wgpu::PipelineCompilationOptions::default(),
            targets: &[Some(options.format.into())],
        }),
        multiview_mask: None,
        cache: None,
    })
}
