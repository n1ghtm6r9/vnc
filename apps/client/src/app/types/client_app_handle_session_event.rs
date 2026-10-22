use log::info;
use rfb_session::SessionEvent;
use winit::dpi::PhysicalSize;
use winit::event_loop::ActiveEventLoop;

use super::ClientApp;
use crate::stats_line;

impl ClientApp {
    pub(super) fn handle_session_event(&mut self, event_loop: &ActiveEventLoop, generation: u64, event: SessionEvent) {
        if !self.session.is_current(generation) {
            return;
        }
        match event {
            SessionEvent::Connected {
                width,
                height,
                name,
                apple_remote_desktop,
            } => {
                self.on_connected(PhysicalSize::new(width, height), &name, apple_remote_desktop);
            }
            SessionEvent::FrameUpdated { patches } => self.upload_patches(&patches),
            SessionEvent::PictureStripe(stripe) => self.upload_stripe(&stripe),
            SessionEvent::Resized { width, height } => self.resize_remote(PhysicalSize::new(width, height)),
            SessionEvent::DisplayGeometry(display) => {
                info!(
                    "[UI] the Mac display has {}x{} pixels for {}x{} points",
                    display.pixel_width, display.pixel_height, display.point_width, display.point_height
                );
                self.mac_display = Some(display);
            }
            SessionEvent::CursorChanged(shape) => self.change_cursor(event_loop, shape),
            SessionEvent::SystemCursorChanged(cursor) => self.change_system_cursor(cursor),
            SessionEvent::CursorVisibilityChanged { visible } => self.set_mac_cursor_visible(visible),
            SessionEvent::ClipboardText(text) => self.receive_clipboard(&text),
            SessionEvent::Stats(stats) => {
                self.stats = Some(stats_line(stats.updates_per_second, stats.bytes_per_second, stats.last_encoding));
            }
            SessionEvent::Disconnected { error } => self.on_disconnected(error),
            SessionEvent::HighPerformance(state) => self.on_high_performance(state),
            SessionEvent::Bell => {}
        }
    }
}
