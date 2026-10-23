use std::time::Instant;

use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;

use super::ClientApp;
use crate::{DISPLAY_RESIZE_DELAY, ScaleMode, overlay_hears_keys, show_app_icon};

impl ClientApp {
    pub(super) fn handle_window_event(&mut self, event_loop: &ActiveEventLoop, event: WindowEvent) {
        let Some(window) = self.window.clone() else {
            return;
        };
        let mut consumed = false;
        let mut repaint = false;
        let key_for_the_mac = matches!(event, WindowEvent::KeyboardInput { .. }) && !overlay_hears_keys(self.link);
        if let Some(egui) = &mut self.egui
            && !key_for_the_mac
        {
            let response = egui.on_window_event(&window, &event);
            consumed = response.consumed;
            repaint = response.repaint;
        }
        if repaint {
            self.request_frame(&window);
        }
        match &event {
            WindowEvent::CloseRequested => {
                self.remember_session_window();
                event_loop.exit();
            }
            WindowEvent::Resized(size) => {
                if let Some(gpu) = &mut self.gpu {
                    gpu.resize(*size);
                }
                self.display_resize_at = Some(Instant::now() + DISPLAY_RESIZE_DELAY);
                window.request_redraw();
            }
            WindowEvent::RedrawRequested => self.render(event_loop, &window),
            WindowEvent::ScaleFactorChanged { scale_factor, .. } => show_app_icon(&window, *scale_factor),
            WindowEvent::Focused(true) => self.send_clipboard(),
            WindowEvent::CursorEntered { .. } => self.pointer_inside = true,
            WindowEvent::CursorLeft { .. } => self.pointer_inside = false,
            WindowEvent::CursorMoved { position, .. } => {
                self.pointer = Some(*position);
                if self.remote.is_some() && self.settings.scale_mode == ScaleMode::Actual {
                    self.request_frame(&window);
                }
            }
            _ => {}
        }
        self.handle_input(&window, &event, consumed);
    }
}
