use winit::dpi::{PhysicalPosition, PhysicalSize};

use crate::{ViewRect, remote_point};

#[test]
fn test_window_offsets_scale_to_remote_pixels() {
    let retina = PhysicalSize::new(3456, 2234);
    let shrunk = ViewRect { x: 100.0, y: 50.0, width: 1728.0, height: 1117.0 };
    assert_eq!(remote_point(PhysicalPosition::new(110.0, 55.0), shrunk, retina), (20, 10), "a shrunk view shows two remote pixels per window pixel");
    assert_eq!(remote_point(PhysicalPosition::new(110.9, 55.9), shrunk, retina), (22, 12), "a fraction of a pixel goes to the nearest remote pixel");
    let enlarged = ViewRect { x: 0.0, y: 0.0, width: 6912.0, height: 4468.0 };
    assert_eq!(remote_point(PhysicalPosition::new(13.0, 7.0), enlarged, retina), (6, 3), "an enlarged view spreads one remote pixel over two window pixels");
}
