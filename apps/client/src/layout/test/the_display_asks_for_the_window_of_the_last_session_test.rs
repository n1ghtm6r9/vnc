use winit::dpi::PhysicalSize;

use crate::{SessionWindow, session_display_size};

#[test]
fn test_the_display_asks_for_the_window_of_the_last_session() {
    let maximized = SessionWindow { width: 2560, height: 1368, maximized: true, fullscreen: false };
    assert_eq!(
        session_display_size(Some(maximized), PhysicalSize::new(2560, 1440)),
        PhysicalSize::new(2560, 1368),
        "the Mac draws a display the size of the window the last session ended with, so its stream does not restart"
    );
}
