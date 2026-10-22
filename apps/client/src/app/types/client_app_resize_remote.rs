use log::info;
use winit::dpi::PhysicalSize;

use super::ClientApp;

impl ClientApp {
    pub(super) fn resize_remote(&mut self, remote: PhysicalSize<u16>) {
        if !self.size_picture(self.media_picture.unwrap_or(remote)) {
            return;
        }
        if self.remote != Some(remote) {
            info!("[UI] the Mac desktop is {}x{}", remote.width, remote.height);
        }
        self.remote = Some(remote);
        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }
}
