use remote_input::ViewRect;
use winit::dpi::{PhysicalPosition, PhysicalSize};

use crate::{ScaleMode, actual_view, fit_view};

pub fn view_for_mode(
    mode: ScaleMode,
    window: PhysicalSize<u32>,
    remote: PhysicalSize<u16>,
    pointer: Option<PhysicalPosition<f64>>,
) -> Option<ViewRect> {
    match mode {
        ScaleMode::Fit => fit_view(window, remote),
        ScaleMode::Actual => actual_view(window, remote, pointer),
    }
}
