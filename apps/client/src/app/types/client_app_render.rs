use std::time::Instant;

use gpu_scaler::ScalerFrame;
use winit::dpi::PhysicalSize;
use winit::event_loop::ActiveEventLoop;
use winit::window::Window;

use super::ClientApp;
use crate::{
    BACKGROUND_COLOR, EguiPaint, Overlay, clear_target, frame_interval, mac_cursor_scale, scale_filter_for_view,
    show_reconnect_panel, surface_view, view_for_mode, viewport_for_view,
};

impl ClientApp {
    pub(super) fn render(&mut self, event_loop: &ActiveEventLoop, window: &Window) {
        let overlay = self.overlay(window);
        let (Some(gpu), Some(egui)) = (&mut self.gpu, &mut self.egui) else {
            return;
        };
        let size = window.inner_size();
        if size.width == 0 || size.height == 0 {
            return;
        }
        gpu.resize(size);
        let Some(surface_texture) = gpu.acquire_frame() else {
            return;
        };
        let target = PhysicalSize::new(surface_texture.texture.width(), surface_texture.texture.height());
        let (form, toolbar) = (&mut self.connect_form, &mut self.toolbar);
        let mut request = None;
        let mut actions = Vec::new();
        let frame = egui.build_frame(window, |ui| match &overlay {
            Overlay::Form => request = form.show(ui),
            Overlay::Toolbar(view) => actions = toolbar.show(ui, view),
            Overlay::Reconnect(view) => actions = show_reconnect_panel(ui, view),
        });
        let refresh = window.current_monitor().and_then(|monitor| monitor.refresh_rate_millihertz());
        self.frame_interval = frame_interval(refresh);
        self.repaint_at = Instant::now().checked_add(frame.repaint_delay.max(self.frame_interval));
        let pointer_over_ui = frame.pointer_over_ui;
        let remote_view = self.remote.and_then(|remote| {
            view_for_mode(self.settings.scale_mode, target, remote, self.pointer).map(|view| (view, remote))
        });
        let image_view = surface_view(&surface_texture, gpu.srgb_format());
        let ui_view = surface_view(&surface_texture, gpu.format());
        let mut encoder = gpu
            .device()
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("frame") });
        match (&mut self.scaler, remote_view) {
            (Some(scaler), Some((view, remote))) => scaler.render(ScalerFrame {
                device: gpu.device(),
                queue: gpu.queue(),
                encoder: &mut encoder,
                target: &image_view,
                target_width: target.width,
                target_height: target.height,
                viewport: viewport_for_view(view),
                filter: scale_filter_for_view(view, self.media_picture.unwrap_or(remote)),
                background: BACKGROUND_COLOR,
            }),
            _ => clear_target(&mut encoder, &image_view, BACKGROUND_COLOR),
        }
        let paint = EguiPaint {
            device: gpu.device(),
            queue: gpu.queue(),
            encoder: &mut encoder,
            target: &ui_view,
            size: target,
        };
        let buffers = egui.paint(paint, frame);
        gpu.queue()
            .submit(buffers.into_iter().chain(std::iter::once(encoder.finish())));
        window.pre_present_notify();
        gpu.queue().present(surface_texture);
        self.last_frame_at = Some(Instant::now());
        if let Some((view, remote)) = remote_view {
            let scale = mac_cursor_scale(view, remote, self.mac_display, self.streamed_display_scale());
            self.follow_view_scale(event_loop, scale);
        }
        self.show_pointer(window, pointer_over_ui);
        for action in actions {
            self.handle_overlay_action(action);
        }
        if let Some(request) = request {
            self.start_connect(request);
        }
    }
}
