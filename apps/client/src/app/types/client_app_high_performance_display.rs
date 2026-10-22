use winit::dpi::PhysicalSize;

use super::ClientApp;
use crate::session_display_size;

impl ClientApp {
    pub(super) fn high_performance_display(&self) -> PhysicalSize<u32> {
        let monitor = self.window.as_ref().and_then(|window| window.current_monitor());
        session_display_size(self.settings.session_window, monitor.map(|monitor| monitor.size()).unwrap_or_default())
    }
}
