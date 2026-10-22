use winit::dpi::PhysicalSize;

use super::ClientApp;
use crate::high_performance_display_size;

impl ClientApp {
    pub(super) fn high_performance_display(&self) -> PhysicalSize<u32> {
        let monitor = self.window.as_ref().and_then(|window| window.current_monitor());
        high_performance_display_size(monitor.map(|monitor| monitor.size()).unwrap_or_default())
    }
}
