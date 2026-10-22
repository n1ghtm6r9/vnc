use winit::dpi::PhysicalSize;

use crate::{SessionWindow, session_display_size};

#[test]
fn test_a_remembered_window_larger_than_the_monitor_is_cut_to_it() {
    let monitor = PhysicalSize::new(2560, 1440);
    let large = SessionWindow { width: 3024, height: 1897, maximized: false, fullscreen: false };
    assert_eq!(
        session_display_size(Some(large), monitor),
        PhysicalSize::new(2560, 1440),
        "a window from a larger monitor asks for no more than this monitor shows"
    );
    let odd = SessionWindow { width: 1365, height: 767, ..large };
    assert_eq!(
        session_display_size(Some(odd), monitor),
        PhysicalSize::new(1364, 766),
        "the size is rounded down to even pixels the video encoder accepts"
    );
    assert_eq!(
        session_display_size(Some(large), PhysicalSize::new(0, 0)),
        PhysicalSize::new(3024, 1896),
        "a monitor of unknown size cuts nothing"
    );
}
