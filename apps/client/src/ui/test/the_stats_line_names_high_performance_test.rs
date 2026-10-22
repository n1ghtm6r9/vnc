use crate::stats_line;

#[test]
fn test_the_stats_line_names_high_performance() {
    assert_eq!(
        stats_line(60.0, 3_000_000, 1010),
        "60 к/с · 24.0 Мбит/с · High Performance",
        "the HEVC stream of the Mac is shown by name"
    );
}
