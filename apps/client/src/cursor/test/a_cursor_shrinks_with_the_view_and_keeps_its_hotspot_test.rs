use rfb_session::CursorShape;

use crate::{ScaledCursor, scale_cursor};

#[test]
fn test_a_cursor_shrinks_with_the_view_and_keeps_its_hotspot() {
    let shape = CursorShape {
        width: 4,
        height: 2,
        hotspot_x: 2,
        hotspot_y: 1,
        rgba: (0..8u8).flat_map(|shade| [shade * 10, shade * 10, shade * 10, 255]).collect(),
    };

    let cursor = scale_cursor(&shape, 0.5);

    assert_eq!(
        cursor,
        Some(ScaledCursor {
            width: 2,
            height: 1,
            hotspot_x: 1,
            hotspot_y: 0,
            rgba: vec![25, 25, 25, 255, 45, 45, 45, 255],
        }),
        "a half-size view halves the cursor, averages each two by two block and moves the hotspot with it"
    );
}
