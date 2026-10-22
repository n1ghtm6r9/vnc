use winit::dpi::{PhysicalPosition, PhysicalSize};

use crate::{ViewRect, remote_point};

#[test]
fn test_a_one_to_one_view_maps_every_pixel_to_itself() {
    let view = ViewRect { x: 40.0, y: 12.0, width: 3456.0, height: 2234.0 };
    let remote = PhysicalSize::new(3456, 2234);
    for column in 0..3456u16 {
        let (x, _) = remote_point(PhysicalPosition::new(40.0 + f64::from(column), 12.0), view, remote);
        assert_eq!(x, column, "window column {column} must land on remote column {column}, not one off through rounding");
    }
    for row in 0..2234u16 {
        let (_, y) = remote_point(PhysicalPosition::new(40.0, 12.0 + f64::from(row)), view, remote);
        assert_eq!(y, row, "window row {row} must land on remote row {row}, not one off through rounding");
    }
}
