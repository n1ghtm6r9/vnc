use winit::dpi::{PhysicalPosition, PhysicalSize};

use crate::actual_view;

#[test]
fn test_actual_size_pans_with_the_pointer() {
    let window = PhysicalSize::new(2560, 1440);
    let remote = PhysicalSize::new(3456u16, 2234u16);
    let top_left = actual_view(window, remote, Some(PhysicalPosition::new(0.0, 0.0))).unwrap();
    assert_eq!((top_left.x, top_left.y), (0.0, 0.0), "the pointer at the top-left corner shows the Mac's top-left corner");
    let bottom_right = actual_view(window, remote, Some(PhysicalPosition::new(2560.0, 1440.0))).unwrap();
    assert_eq!((bottom_right.x, bottom_right.y), (-896.0, -794.0), "the pointer at the far corner shows the Mac's far corner");
    let centre = actual_view(window, remote, None).unwrap();
    assert_eq!((centre.x, centre.y), (-448.0, -397.0), "without a pointer the middle of the Mac is shown");
    assert_eq!((centre.width, centre.height), (3456.0, 2234.0), "1:1 keeps every remote pixel");
    let small = actual_view(window, PhysicalSize::new(1280, 800), Some(PhysicalPosition::new(0.0, 0.0))).unwrap();
    assert_eq!((small.x, small.y), (640.0, 320.0), "a screen smaller than the window is centred and does not pan");
}
