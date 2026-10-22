use log::warn;
use winit::dpi::PhysicalSize;

use super::ClientApp;

impl ClientApp {
    pub(super) fn show_media_picture(&mut self, width: u32, height: u32) {
        let (Ok(picture_width), Ok(picture_height)) = (u16::try_from(width), u16::try_from(height)) else {
            warn!("[UI] a {width}x{height} High Performance picture is too large to show");
            self.reconnect_without_high_performance();
            return;
        };
        let picture = PhysicalSize::new(picture_width, picture_height);
        self.media_picture = Some(picture);
        if self.size_picture(picture)
            && let Some(window) = &self.window
        {
            window.request_redraw();
        }
    }
}
