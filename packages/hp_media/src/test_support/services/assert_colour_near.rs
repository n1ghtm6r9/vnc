use crate::test_support::COLOUR_TOLERANCE;

pub fn assert_colour_near(actual: [u8; 3], expected: [u8; 3], what: &str) {
    let near = actual
        .iter()
        .zip(expected)
        .all(|(&channel, wanted)| channel.abs_diff(wanted) <= COLOUR_TOLERANCE);
    assert!(near, "{what} is {actual:?}, expected near {expected:?}");
}
