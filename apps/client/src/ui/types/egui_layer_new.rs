use egui::ViewportId;
use winit::window::Window;

use super::EguiLayer;

impl EguiLayer {
    pub fn new(window: &Window, device: &wgpu::Device, format: wgpu::TextureFormat) -> EguiLayer {
        let context = egui::Context::default();
        let max_texture_side = device.limits().max_texture_dimension_2d as usize;
        let state = egui_winit::State::new(
            context.clone(),
            ViewportId::ROOT,
            window,
            Some(window.scale_factor() as f32),
            window.theme(),
            Some(max_texture_side),
        );
        let renderer = egui_wgpu::Renderer::new(device, format, egui_wgpu::RendererOptions::default());
        EguiLayer {
            context,
            state,
            renderer,
        }
    }
}
