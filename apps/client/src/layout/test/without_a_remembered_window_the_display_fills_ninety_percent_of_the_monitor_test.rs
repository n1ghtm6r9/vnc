use winit::dpi::PhysicalSize;

use crate::{SessionWindow, session_display_size};

#[test]
fn test_without_a_remembered_window_the_display_fills_ninety_percent_of_the_monitor() {
    let monitor = PhysicalSize::new(2560, 1440);
    assert_eq!(
        session_display_size(None, monitor),
        PhysicalSize::new(2304, 1296),
        "the first session asks for nine tenths of the monitor"
    );
    let empty = SessionWindow { width: 0, height: 1368, maximized: true, fullscreen: false };
    assert_eq!(
        session_display_size(Some(empty), monitor),
        PhysicalSize::new(2304, 1296),
        "a remembered window without a size counts as none"
    );
}
