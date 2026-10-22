use crate::test_support::COLOUR_TOLERANCE;

pub fn rgb_near(actual: [u8; 3], expected: [u8; 3]) -> bool {
    actual
        .iter()
        .zip(expected)
        .all(|(&channel, wanted)| channel.abs_diff(wanted) <= COLOUR_TOLERANCE)
}
