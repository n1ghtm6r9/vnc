use winit::dpi::PhysicalSize;

use crate::remembered_window;

#[test]
fn test_a_minimized_window_is_not_remembered() {
    assert_eq!(
        remembered_window(PhysicalSize::new(0, 0), true, false),
        None,
        "a minimized window has no size, so the window remembered before stays"
    );
}
