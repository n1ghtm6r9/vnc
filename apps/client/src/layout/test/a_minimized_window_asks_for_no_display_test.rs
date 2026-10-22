use winit::dpi::PhysicalSize;

use crate::window_display_size;

#[test]
fn test_a_minimized_window_asks_for_no_display() {
    assert_eq!(window_display_size(PhysicalSize::new(0, 0)), None, "a minimized window keeps the display the Mac has");
    assert_eq!(
        window_display_size(PhysicalSize::new(1, 900)),
        None,
        "a window too narrow for a display keeps the display the Mac has"
    );
}
