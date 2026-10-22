use log::warn;
use winit::event_loop::ActiveEventLoop;
use winit::window::CustomCursor;

use super::ClientApp;
use crate::{RemoteCursor, scale_cursor};

impl ClientApp {
    pub(super) fn rebuild_cursor(&mut self, event_loop: &ActiveEventLoop, scale: f64) {
        self.cursor_scale = scale;
        let Some(shape) = &self.cursor_shape else {
            return;
        };
        let Some(cursor) = scale_cursor(shape, scale) else {
            self.remote_cursor = Some(RemoteCursor::Hidden);
            return;
        };
        let source = CustomCursor::from_rgba(
            cursor.rgba,
            cursor.width,
            cursor.height,
            cursor.hotspot_x,
            cursor.hotspot_y,
        );
        self.remote_cursor = match source {
            Ok(source) => Some(RemoteCursor::Shown(event_loop.create_custom_cursor(source))),
            Err(error) => {
                warn!("[UI] the cursor of the Mac cannot be shown: {error}");
                None
            }
        };
    }
}
