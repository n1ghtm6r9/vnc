use winit::dpi::{PhysicalPosition, PhysicalSize};

pub fn centred_position(
    monitor_position: PhysicalPosition<i32>,
    monitor_size: PhysicalSize<u32>,
    window: PhysicalSize<u32>,
) -> PhysicalPosition<i32> {
    let offset = |monitor: u32, window: u32| i32::try_from(monitor.saturating_sub(window) / 2).unwrap_or(0);
    PhysicalPosition::new(
        monitor_position.x.saturating_add(offset(monitor_size.width, window.width)),
        monitor_position.y.saturating_add(offset(monitor_size.height, window.height)),
    )
}
