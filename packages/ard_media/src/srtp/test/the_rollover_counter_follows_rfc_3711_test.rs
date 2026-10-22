use crate::estimate_roc;

#[test]
fn test_the_rollover_counter_follows_rfc_3711() {
    let cases = [
        (
            None,
            40000,
            0,
            "the first packet of a stream starts the count at zero",
        ),
        (Some(0x0001_0005), 10, 1, "a nearby number keeps the count"),
        (
            Some(0x0000_fff0),
            3,
            1,
            "a small number after a high one has wrapped",
        ),
        (
            Some(0x0001_0003),
            0xfff0,
            0,
            "a high number just after a wrap is a late one from before it",
        ),
        (
            Some(0x0000_0003),
            0xfff0,
            0,
            "the count never goes below zero",
        ),
        (
            Some(0x0002_9000),
            0x0800,
            3,
            "more than half the space behind a high number is the next cycle",
        ),
        (
            Some(0x0002_9000),
            0x1001,
            2,
            "less than half the space behind a high number is the same cycle",
        ),
    ];
    for (highest, seq, expected, meaning) in cases {
        assert_eq!(estimate_roc(highest, seq), expected, "{meaning}");
    }
}
