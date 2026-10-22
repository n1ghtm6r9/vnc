use log::info;

use super::ClientApp;

impl ClientApp {
    pub(super) fn set_mac_cursor_visible(&mut self, visible: bool) {
        info!("[UI] the Mac {} its cursor", if visible { "shows" } else { "hides" });
        self.mac_cursor_hidden = !visible;
        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }
}
