use winit::event::WindowEvent;
use winit::window::Window;

use super::EguiLayer;

impl EguiLayer {
    pub fn on_window_event(&mut self, window: &Window, event: &WindowEvent) -> egui_winit::EventResponse {
        self.state.on_window_event(window, event)
    }
}
