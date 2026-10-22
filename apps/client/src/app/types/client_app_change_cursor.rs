use rfb_session::CursorShape;
use winit::event_loop::ActiveEventLoop;

use super::ClientApp;
use crate::mac_cursor_scale;

impl ClientApp {
    pub(super) fn change_cursor(&mut self, event_loop: &ActiveEventLoop, shape: CursorShape) {
        let Some(window) = self.window.clone() else {
            return;
        };
        let scale = self
            .current_view(&window)
            .zip(self.remote)
            .map_or(1.0, |(view, remote)| {
                mac_cursor_scale(view, remote, self.mac_display, self.streamed_display_scale())
            });
        self.cursor_shape = Some(shape);
        self.rebuild_cursor(event_loop, scale);
        window.request_redraw();
    }
}
