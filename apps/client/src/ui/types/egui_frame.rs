use std::time::Duration;

pub struct EguiFrame {
    pub primitives: Vec<egui::ClippedPrimitive>,
    pub textures_delta: egui::TexturesDelta,
    pub pixels_per_point: f32,
    pub repaint_delay: Duration,
    pub pointer_over_ui: bool,
}
