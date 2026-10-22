use remote_input::ViewRect;
use winit::dpi::PhysicalSize;

use crate::mac_cursor_scale;

#[test]
fn test_a_retina_stream_draws_the_cursor_at_the_display_scale() {
    let view = ViewRect {
        x: 0.0,
        y: 0.0,
        width: 2880.0,
        height: 1620.0,
    };

    let scale = mac_cursor_scale(view, PhysicalSize::new(3840, 2160), None, Some(2));

    assert_eq!(
        scale, 1.5,
        "the Mac sends its High Performance cursor in points, so a Retina desktop of 3840 pixels shown 2880 wide draws it at 1.5"
    );
}
