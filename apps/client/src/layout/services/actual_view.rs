use remote_input::ViewRect;
use winit::dpi::{PhysicalPosition, PhysicalSize};

use crate::pan_offset;

pub fn actual_view(
    window: PhysicalSize<u32>,
    remote: PhysicalSize<u16>,
    pointer: Option<PhysicalPosition<f64>>,
) -> Option<ViewRect> {
    if window.width == 0 || window.height == 0 || remote.width == 0 || remote.height == 0 {
        return None;
    }
    Some(ViewRect {
        x: pan_offset(window.width, remote.width, pointer.map(|position| position.x)),
        y: pan_offset(window.height, remote.height, pointer.map(|position| position.y)),
        width: f64::from(remote.width),
        height: f64::from(remote.height),
    })
}
