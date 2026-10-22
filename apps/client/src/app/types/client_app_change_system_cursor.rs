use rfb_session::SystemCursor;

use super::ClientApp;
use crate::{RemoteCursor, system_cursor_icon};

impl ClientApp {
    pub(super) fn change_system_cursor(&mut self, cursor: SystemCursor) {
        self.cursor_shape = None;
        self.remote_cursor = Some(RemoteCursor::System(system_cursor_icon(cursor)));
        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }
}
