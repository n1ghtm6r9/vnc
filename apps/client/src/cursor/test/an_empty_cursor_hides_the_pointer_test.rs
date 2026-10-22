use rfb_session::CursorShape;

use crate::scale_cursor;

#[test]
fn test_an_empty_cursor_hides_the_pointer() {
    let shape = CursorShape {
        width: 0,
        height: 0,
        hotspot_x: 0,
        hotspot_y: 0,
        rgba: Vec::new(),
    };

    let cursor = scale_cursor(&shape, 1.0);

    assert_eq!(cursor, None, "a 0x0 cursor from the Mac hides the pointer");
}
