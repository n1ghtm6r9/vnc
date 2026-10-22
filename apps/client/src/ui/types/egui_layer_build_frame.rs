use std::time::Duration;

use egui::ViewportId;
use winit::window::Window;

use super::{EguiFrame, EguiLayer};

impl EguiLayer {
    pub fn build_frame(&mut self, window: &Window, run_ui: impl FnMut(&mut egui::Ui)) -> EguiFrame {
        let input = self.state.take_egui_input(window);
        let output = self.context.run_ui(input, run_ui);
        self.state.handle_platform_output(window, output.platform_output);
        let repaint_delay = output
            .viewport_output
            .get(&ViewportId::ROOT)
            .map(|viewport| viewport.repaint_delay)
            .unwrap_or(Duration::MAX);
        let primitives = self.context.tessellate(output.shapes, output.pixels_per_point);
        EguiFrame {
            primitives,
            textures_delta: output.textures_delta,
            pixels_per_point: output.pixels_per_point,
            repaint_delay,
            pointer_over_ui: self.context.egui_wants_pointer_input(),
        }
    }
}
