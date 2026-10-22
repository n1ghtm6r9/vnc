use crate::stats_line;

#[test]
fn test_the_stats_line_names_the_apple_mvs_encoding() {
    assert_eq!(stats_line(60.0, 1_000_000, 1011), "60 к/с · 8.0 Мбит/с · Apple MVS", "the Mac's own encoding is shown by name");
}
