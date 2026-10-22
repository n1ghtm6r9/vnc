use super::assert_frame_matches;
use crate::{
    DecodedFrame,
    test_support::{BLACK, BLUE, RED, SPLIT_COLUMN, SPLIT_ROW, WHITE},
};

pub fn assert_four_colour_frames(frames: &[DecodedFrame]) {
    assert_eq!(
        frames.len(),
        4,
        "every picture of the stream comes out as a frame"
    );
    assert_frame_matches(&frames[0], |_, _| WHITE, "the first frame");
    assert_frame_matches(&frames[1], |_, _| BLACK, "the second frame");
    assert_frame_matches(
        &frames[2],
        |_, y| if y < SPLIT_ROW { RED } else { BLUE },
        "the third frame",
    );
    assert_frame_matches(
        &frames[3],
        |x, _| if x < SPLIT_COLUMN { RED } else { BLUE },
        "the fourth frame",
    );
}
