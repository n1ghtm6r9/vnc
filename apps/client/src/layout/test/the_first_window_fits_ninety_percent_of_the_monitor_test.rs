use winit::dpi::PhysicalSize;

use crate::initial_window_size;

#[test]
fn test_the_first_window_fits_ninety_percent_of_the_monitor() {
    let monitor = PhysicalSize::new(2560, 1440);
    let size = initial_window_size(monitor, PhysicalSize::new(3456, 2234));
    assert_eq!(size, PhysicalSize::new(2005, 1296), "the Retina screen is shrunk into 90% of the monitor with its aspect kept");
    let small = initial_window_size(monitor, PhysicalSize::new(1280, 800));
    assert_eq!(small, PhysicalSize::new(1280, 800), "a screen that already fits opens at 1:1");
}
