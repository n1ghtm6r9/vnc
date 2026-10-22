use std::sync::Arc;

use log::info;
use winit::window::Window;

use super::{GpuContext, GpuError};
use crate::{choose_present_mode, choose_surface_format};

impl GpuContext {
    pub(super) fn open(window: Arc<Window>, backends: wgpu::Backends) -> Result<GpuContext, GpuError> {
        let size = window.inner_size();
        let mut descriptor = wgpu::InstanceDescriptor::new_without_display_handle_from_env();
        descriptor.backends = backends;
        let instance = wgpu::Instance::new(descriptor);
        let surface = instance
            .create_surface(window)
            .map_err(|error| GpuError::Surface(error.to_string()))?;
        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            force_fallback_adapter: false,
            compatible_surface: Some(&surface),
            ..Default::default()
        }))
        .map_err(|error| GpuError::Adapter(error.to_string()))?;
        let (device, queue) = pollster::block_on(adapter.request_device(&wgpu::DeviceDescriptor {
            label: Some("vnc"),
            ..Default::default()
        }))
        .map_err(|error| GpuError::Device(error.to_string()))?;
        let capabilities = surface.get_capabilities(&adapter);
        let format = choose_surface_format(&capabilities.formats)
            .ok_or_else(|| GpuError::Surface("the surface offers no texture format".to_string()))?;
        let srgb_format = format.add_srgb_suffix();
        let view_formats = if srgb_format == format {
            Vec::new()
        } else {
            vec![srgb_format]
        };
        let present_mode = choose_present_mode(&capabilities.present_modes);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            color_space: wgpu::SurfaceColorSpace::Auto,
            width: size.width.max(1),
            height: size.height.max(1),
            present_mode,
            desired_maximum_frame_latency: 2,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats,
        };
        surface.configure(&device, &config);
        let info = adapter.get_info();
        info!(
            "[GPU] {} via {:?}, surface {:?}, {:?}",
            info.name, info.backend, format, present_mode
        );
        Ok(GpuContext {
            surface,
            device,
            queue,
            config,
        })
    }
}
