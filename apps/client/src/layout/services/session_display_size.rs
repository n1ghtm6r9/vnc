use winit::dpi::PhysicalSize;

use crate::{SessionWindow, high_performance_display_size, window_display_size};

pub fn session_display_size(remembered: Option<SessionWindow>, monitor: PhysicalSize<u32>) -> PhysicalSize<u32> {
    let fit = |side: u32, limit: u32| if limit == 0 { side } else { side.min(limit) };
    remembered
        .map(|window| PhysicalSize::new(fit(window.width, monitor.width), fit(window.height, monitor.height)))
        .and_then(window_display_size)
        .unwrap_or_else(|| high_performance_display_size(monitor))
}
