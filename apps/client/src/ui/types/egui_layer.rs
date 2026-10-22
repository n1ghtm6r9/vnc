pub struct EguiLayer {
    pub(super) context: egui::Context,
    pub(super) state: egui_winit::State,
    pub(super) renderer: egui_wgpu::Renderer,
}
