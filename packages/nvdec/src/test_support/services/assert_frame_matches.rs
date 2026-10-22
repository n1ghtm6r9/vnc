use super::{frame_rgb, rgb_near};
use crate::{
    DecodedFrame,
    test_support::{FIXTURE_HEIGHT, FIXTURE_WIDTH},
};

pub fn assert_frame_matches(
    frame: &DecodedFrame,
    expected: impl Fn(u32, u32) -> [u8; 3],
    what: &str,
) {
    assert_eq!(
        (frame.width, frame.height),
        (FIXTURE_WIDTH, FIXTURE_HEIGHT),
        "{what} is cropped to the display area of its picture"
    );
    let chroma = frame
        .layout
        .chroma_planes(frame.width as usize, frame.height as usize);
    assert_eq!(
        frame.planes.len(),
        (FIXTURE_WIDTH * FIXTURE_HEIGHT) as usize + chroma.count * chroma.row_bytes * chroma.rows,
        "{what} holds its luma plane and its chroma planes"
    );
    for y in 0..frame.height {
        for x in 0..frame.width {
            let actual = frame_rgb(frame, x, y);
            let wanted = expected(x, y);
            assert!(
                rgb_near(actual, wanted),
                "{what} is {actual:?} at ({x}, {y}) where about {wanted:?} was painted"
            );
        }
    }
}
