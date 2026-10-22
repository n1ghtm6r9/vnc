use winit::dpi::PhysicalSize;

use crate::{high_performance_display_size, initial_window_size};

#[test]
fn test_the_high_performance_display_fills_ninety_percent_of_the_monitor() {
    let monitor = PhysicalSize::new(2560, 1440);
    let display = high_performance_display_size(monitor);
    assert_eq!(display, PhysicalSize::new(2304, 1296), "the Mac draws a virtual display as large as the first window");
    let odd_monitor = PhysicalSize::new(1366, 768);
    let rounded = high_performance_display_size(odd_monitor);
    assert_eq!(rounded, PhysicalSize::new(1228, 690), "the size is rounded down to even pixels the video encoder accepts");
    let window = initial_window_size(odd_monitor, PhysicalSize::new(1228, 690));
    assert_eq!(window, PhysicalSize::new(1228, 690), "the first window shows the virtual display 1:1");
}
