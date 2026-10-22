use winit::dpi::PhysicalSize;

use crate::INITIAL_MONITOR_FRACTION;

pub fn high_performance_display_size(monitor: PhysicalSize<u32>) -> PhysicalSize<u32> {
    let even_share = |side: u32| (f64::from(side) * INITIAL_MONITOR_FRACTION / 2.0).floor() as u32 * 2;
    PhysicalSize::new(even_share(monitor.width), even_share(monitor.height))
}
