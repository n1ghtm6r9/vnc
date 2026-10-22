use winit::window::CursorIcon;

use super::ClientApp;

impl ClientApp {
    pub(super) fn restore_default_pointer(&mut self) {
        self.cursor_shape = None;
        self.remote_cursor = None;
        self.cursor_scale = 1.0;
        self.cursor_on_remote = false;
        self.mac_cursor_hidden = false;
        let Some(window) = &self.window else {
            return;
        };
        window.set_cursor_visible(true);
        if self.pointer_inside {
            window.set_cursor(CursorIcon::Default);
        }
    }
}
