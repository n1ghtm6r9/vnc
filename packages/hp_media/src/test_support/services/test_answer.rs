use super::answer_payload;
use crate::test_support::{TEST_PICTURE_HEIGHT, TEST_PICTURE_WIDTH, TEST_STRIPES};

pub fn test_answer() -> Vec<u8> {
    answer_payload(TEST_PICTURE_WIDTH, TEST_PICTURE_HEIGHT, TEST_STRIPES)
}
