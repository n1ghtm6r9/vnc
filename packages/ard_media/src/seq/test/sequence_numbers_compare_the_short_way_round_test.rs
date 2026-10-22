use crate::seq_delta;

#[test]
fn test_sequence_numbers_compare_the_short_way_round() {
    let cases = [
        (5, 3, 2, "a later number is ahead"),
        (3, 5, -2, "an earlier number is behind"),
        (1, 65535, 2, "a number past the wrap is ahead"),
        (65535, 1, -2, "a number before the wrap is behind"),
        (32768, 0, -32768, "half the space away counts as behind"),
        (70000, 4464, 0, "an extended value equals its wire value"),
    ];
    for (to, from, expected, meaning) in cases {
        assert_eq!(seq_delta(to, from), expected, "{meaning}");
    }
}
