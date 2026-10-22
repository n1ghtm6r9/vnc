use winit::dpi::PhysicalSize;

use crate::INITIAL_MONITOR_FRACTION;

pub fn initial_window_size(monitor: PhysicalSize<u32>, remote: PhysicalSize<u16>) -> PhysicalSize<u32> {
    let remote_width = f64::from(remote.width.max(1));
    let remote_height = f64::from(remote.height.max(1));
    if monitor.width == 0 || monitor.height == 0 {
        return PhysicalSize::new(remote_width as u32, remote_height as u32);
    }
    let max_width = f64::from(monitor.width) * INITIAL_MONITOR_FRACTION;
    let max_height = f64::from(monitor.height) * INITIAL_MONITOR_FRACTION;
    let scale = (max_width / remote_width).min(max_height / remote_height).min(1.0);
    PhysicalSize::new(
        (remote_width * scale).round().max(1.0) as u32,
        (remote_height * scale).round().max(1.0) as u32,
    )
}
