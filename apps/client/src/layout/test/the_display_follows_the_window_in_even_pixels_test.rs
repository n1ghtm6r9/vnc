use winit::dpi::PhysicalSize;

use crate::window_display_size;

#[test]
fn test_the_display_follows_the_window_in_even_pixels() {
    assert_eq!(
        window_display_size(PhysicalSize::new(2560, 1440)),
        Some(PhysicalSize::new(2560, 1440)),
        "a full screen window asks for a display as large as the monitor"
    );
    assert_eq!(
        window_display_size(PhysicalSize::new(1601, 901)),
        Some(PhysicalSize::new(1600, 900)),
        "the size is rounded down to even pixels the video encoder accepts"
    );
}
