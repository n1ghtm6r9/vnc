use super::{GpuScaler, PassParams, PipelineOptions};
use crate::{
    ConvertParams, HORIZONTAL_ENTRY, INTERMEDIATE_FORMAT, NEAREST_ENTRY, RESAMPLE_WGSL,
    VERTICAL_ENTRY, create_bind_group_layout, create_convert_layout, create_convert_pipeline,
    create_params_buffer, create_pipeline,
};

impl GpuScaler {
    pub fn new(device: &wgpu::Device, target_format: wgpu::TextureFormat) -> GpuScaler {
        let layout = create_bind_group_layout(device);
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("gpu scaler pipeline layout"),
            bind_group_layouts: &[Some(&layout)],
            immediate_size: 0,
        });
        let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("gpu scaler resample"),
            source: wgpu::ShaderSource::Wgsl(RESAMPLE_WGSL.into()),
        });
        let pipeline = |fragment_entry, format| {
            create_pipeline(
                device,
                PipelineOptions {
                    layout: &pipeline_layout,
                    module: &module,
                    fragment_entry,
                    format,
                },
            )
        };
        let horizontal_pipeline = pipeline(HORIZONTAL_ENTRY, INTERMEDIATE_FORMAT);
        let vertical_pipeline = pipeline(VERTICAL_ENTRY, target_format);
        let nearest_pipeline = pipeline(NEAREST_ENTRY, target_format);
        let convert_layout = create_convert_layout(device);
        let convert_pipeline = create_convert_pipeline(device, &convert_layout);
        let params = |label| create_params_buffer(device, label, size_of::<PassParams>());
        log::info!("[GPU] scaler ready for {target_format:?} targets");
        GpuScaler {
            layout,
            horizontal_pipeline,
            vertical_pipeline,
            nearest_pipeline,
            convert_layout,
            convert_pipeline,
            convert_params: create_params_buffer(
                device,
                "gpu scaler yuv params",
                size_of::<ConvertParams>(),
            ),
            source_params: params("gpu scaler source params"),
            intermediate_params: params("gpu scaler intermediate params"),
            source: None,
            intermediate: None,
            yuv: None,
            encode_srgb: !target_format.is_srgb(),
            max_dimension: device.limits().max_texture_dimension_2d,
        }
    }
}
