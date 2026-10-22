use rfb_session::CursorShape;

use crate::{MAX_CURSOR_LENGTH, scale_cursor};

#[test]
fn test_a_cursor_never_grows_past_the_largest_size() {
    let shape = CursorShape {
        width: 4,
        height: 2,
        hotspot_x: 1,
        hotspot_y: 1,
        rgba: vec![255; 4 * 2 * 4],
    };

    let cursor = scale_cursor(&shape, 200.0).expect("a visible cursor stays visible");

    assert_eq!(
        (cursor.width, cursor.height),
        (MAX_CURSOR_LENGTH, MAX_CURSOR_LENGTH / 2),
        "a tiny desktop in a big window gives a cursor no longer than the largest side, with its proportions kept"
    );
    assert_eq!(
        (cursor.hotspot_x, cursor.hotspot_y),
        (64, 64),
        "the hotspot follows the reduced scale"
    );
    assert_eq!(
        cursor.rgba.len(),
        usize::from(cursor.width) * usize::from(cursor.height) * 4,
        "the image holds every pixel of the capped cursor"
    );
}
