pub(crate) struct PipelineOptions<'a> {
    pub layout: &'a wgpu::PipelineLayout,
    pub module: &'a wgpu::ShaderModule,
    pub fragment_entry: &'a str,
    pub format: wgpu::TextureFormat,
}
