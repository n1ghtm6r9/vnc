use winit::dpi::{PhysicalPosition, PhysicalSize};

use crate::{ViewRect, remote_point};

#[test]
fn test_an_empty_view_or_remote_maps_to_the_origin() {
    let full_hd = PhysicalSize::new(1920, 1080);
    assert_eq!(remote_point(PhysicalPosition::new(10.0, 10.0), ViewRect::default(), full_hd), (0, 0), "an empty view maps every point to the origin instead of dividing by zero");
    let single_pixel = ViewRect { x: 0.0, y: 0.0, width: 1.0, height: 1.0 };
    assert_eq!(remote_point(PhysicalPosition::new(0.0, 0.0), single_pixel, full_hd), (0, 0), "a one-pixel view maps to the origin instead of dividing by zero");
    let inverted = ViewRect { x: 0.0, y: 0.0, width: -10.0, height: -10.0 };
    assert_eq!(remote_point(PhysicalPosition::new(-5.0, -5.0), inverted, full_hd), (0, 0), "a view with a negative size maps to the origin");
    let view = ViewRect { x: 0.0, y: 0.0, width: 800.0, height: 600.0 };
    assert_eq!(remote_point(PhysicalPosition::new(400.0, 300.0), view, PhysicalSize::new(0, 0)), (0, 0), "a remote screen without pixels maps to the origin");
    assert_eq!(remote_point(PhysicalPosition::new(f64::NAN, f64::NAN), view, full_hd), (0, 0), "a position that is not a number maps to the origin");
}
