use remote_input::ViewRect;
use winit::window::Window;

use super::ClientApp;
use crate::view_for_mode;

impl ClientApp {
    pub(super) fn current_view(&self, window: &Window) -> Option<ViewRect> {
        view_for_mode(self.settings.scale_mode, window.inner_size(), self.remote?, self.pointer)
    }
}
