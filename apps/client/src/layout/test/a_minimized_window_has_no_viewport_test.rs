use winit::dpi::{PhysicalPosition, PhysicalSize};

use crate::{actual_view, fit_view};

#[test]
fn test_a_minimized_window_has_no_viewport() {
    let remote = PhysicalSize::new(3456u16, 2234u16);
    assert!(fit_view(PhysicalSize::new(0, 0), remote).is_none(), "a minimized window draws nothing in fit mode");
    assert!(fit_view(PhysicalSize::new(800, 0), remote).is_none(), "a window with no height draws nothing");
    let pointer = Some(PhysicalPosition::new(10.0, 10.0));
    assert!(actual_view(PhysicalSize::new(0, 0), remote, pointer).is_none(), "a minimized window draws nothing at 1:1");
    assert!(fit_view(PhysicalSize::new(800, 600), PhysicalSize::new(0, 0)).is_none(), "a server without a screen draws nothing");
}
