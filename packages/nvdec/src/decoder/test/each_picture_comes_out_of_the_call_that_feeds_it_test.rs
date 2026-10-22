use crate::test_support::{FOUR_COLOURS_444, access_units, test_decoder};

#[test]
fn test_each_picture_comes_out_of_the_call_that_feeds_it() {
    let Some(mut decoder) = test_decoder() else {
        return;
    };
    let units = access_units(FOUR_COLOURS_444);
    assert_eq!(units.len(), 4, "the fixture holds four pictures");

    let counts: Vec<usize> = units
        .iter()
        .map(|unit| decoder.decode(unit).expect("a clean picture decodes").len())
        .collect();

    assert_eq!(
        counts,
        [1, 1, 1, 1],
        "no picture waits for the next one before it is shown"
    );
}
