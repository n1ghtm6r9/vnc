use rfb_session::CursorShape;

use crate::scale_cursor;

#[test]
fn test_a_shrunk_cursor_keeps_the_colour_of_its_soft_edges() {
    let shape = CursorShape {
        width: 4,
        height: 1,
        hotspot_x: 0,
        hotspot_y: 0,
        rgba: vec![255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    };

    let cursor = scale_cursor(&shape, 0.25).expect("a visible cursor stays visible");

    assert_eq!(
        cursor.rgba,
        vec![255, 255, 255, 64],
        "a white pixel merged with three transparent ones stays white and becomes a quarter opaque, not grey"
    );
}
