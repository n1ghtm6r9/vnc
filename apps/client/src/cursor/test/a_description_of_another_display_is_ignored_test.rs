use remote_input::ViewRect;
use rfb_session::AppleDisplayGeometry;
use winit::dpi::PhysicalSize;

use crate::mac_cursor_scale;

#[test]
fn test_a_description_of_another_display_is_ignored() {
    let view = ViewRect {
        x: 0.0,
        y: 0.0,
        width: 2880.0,
        height: 1620.0,
    };
    let remote = PhysicalSize::new(3840, 2160);
    let display = AppleDisplayGeometry {
        pixel_width: 3024,
        pixel_height: 1964,
        point_width: 1512,
        point_height: 982,
    };

    assert_eq!(
        mac_cursor_scale(view, remote, Some(display), None),
        0.75,
        "a display of another size is not the one in the picture, so it says nothing about the cursor"
    );
    assert_eq!(
        mac_cursor_scale(view, remote, Some(display), Some(2)),
        1.5,
        "the scale of the streamed display applies instead"
    );
}
