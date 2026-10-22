use winit::event_loop::ActiveEventLoop;

use super::ClientApp;
use crate::CURSOR_SCALE_TOLERANCE;

impl ClientApp {
    pub(super) fn follow_view_scale(&mut self, event_loop: &ActiveEventLoop, scale: f64) {
        if (scale / self.cursor_scale - 1.0).abs() > CURSOR_SCALE_TOLERANCE {
            self.rebuild_cursor(event_loop, scale);
        }
    }
}
