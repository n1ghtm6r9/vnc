use super::ClientApp;
use crate::remembered_window;

impl ClientApp {
    pub(super) fn remember_session_window(&mut self) {
        let Some(window) = &self.window else {
            return;
        };
        if self.remote.is_none() {
            return;
        }
        let remembered = remembered_window(window.inner_size(), window.is_maximized(), window.fullscreen().is_some());
        if remembered.is_none() || remembered == self.settings.session_window {
            return;
        }
        self.settings.session_window = remembered;
        self.save_settings();
    }
}
