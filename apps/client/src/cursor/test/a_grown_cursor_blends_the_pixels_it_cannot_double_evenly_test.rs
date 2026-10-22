use rfb_session::CursorShape;

use crate::scale_cursor;

#[test]
fn test_a_grown_cursor_blends_the_pixels_it_cannot_double_evenly() {
    let shape = CursorShape {
        width: 5,
        height: 1,
        hotspot_x: 0,
        hotspot_y: 0,
        rgba: [0, 255, 0, 255, 0].iter().flat_map(|&shade| [shade, shade, shade, 255]).collect(),
    };

    let cursor = scale_cursor(&shape, 1.2).expect("a visible cursor stays visible");
    let shades: Vec<u8> = cursor.rgba.iter().step_by(4).copied().collect();

    assert_eq!(
        shades,
        vec![0, 204, 102, 102, 204, 0],
        "five pixels grown to six mix the pixels that straddle two source pixels instead of doubling one of them"
    );
}
