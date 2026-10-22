use std::time::Instant;

use winit::window::Window;

use super::ClientApp;
use crate::frame_wait_until;

impl ClientApp {
    pub(super) fn request_frame(&mut self, window: &Window) {
        match frame_wait_until(self.last_frame_at, self.frame_interval, Instant::now()) {
            None => window.request_redraw(),
            Some(at) => self.repaint_at = Some(self.repaint_at.map_or(at, |current| current.min(at))),
        }
    }
}
