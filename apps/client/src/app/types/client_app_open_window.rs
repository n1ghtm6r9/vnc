use std::sync::Arc;

use gpu_scaler::GpuScaler;
use log::error;
use winit::dpi::LogicalSize;
use winit::event_loop::ActiveEventLoop;
use winit::window::Window;

use super::ClientApp;
use crate::{EguiLayer, FORM_WINDOW_SIZE, GpuContext, WINDOW_TITLE, install_keyboard_grab, show_app_icon};

impl ClientApp {
    pub(super) fn open_window(&mut self, event_loop: &ActiveEventLoop) {
        let attributes = Window::default_attributes()
            .with_title(WINDOW_TITLE)
            .with_inner_size(FORM_WINDOW_SIZE)
            .with_min_inner_size(LogicalSize::new(380.0, 320.0));
        let window = match event_loop.create_window(attributes) {
            Ok(window) => Arc::new(window),
            Err(error) => {
                error!("[UI] the window could not be created: {error}");
                self.failed = true;
                event_loop.exit();
                return;
            }
        };
        show_app_icon(&window, window.scale_factor());
        let gpu = match GpuContext::new(window.clone()) {
            Ok(gpu) => gpu,
            Err(error) => {
                error!("[GPU] {error}");
                self.failed = true;
                event_loop.exit();
                return;
            }
        };
        self.egui = Some(EguiLayer::new(&window, gpu.device(), gpu.format()));
        self.scaler = Some(GpuScaler::new(gpu.device(), gpu.srgb_format()));
        self.gpu = Some(gpu);
        if self.settings.grab_keyboard {
            self.keyboard_grab = install_keyboard_grab(&window, self.proxy.clone());
        }
        window.request_redraw();
        self.window = Some(window);
    }
}
