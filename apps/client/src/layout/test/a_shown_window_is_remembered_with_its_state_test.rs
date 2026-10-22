use winit::dpi::PhysicalSize;

use crate::{SessionWindow, remembered_window};

#[test]
fn test_a_shown_window_is_remembered_with_its_state() {
    assert_eq!(
        remembered_window(PhysicalSize::new(2560, 1368), true, false),
        Some(SessionWindow { width: 2560, height: 1368, maximized: true, fullscreen: false }),
        "the window is remembered with its size and whether it was maximized or full screen"
    );
}
