use crate::stats_line;

#[test]
fn test_the_stats_line_shows_fps_bitrate_and_encoding() {
    assert_eq!(stats_line(59.6, 2_500_000, 16), "60 к/с · 20.0 Мбит/с · ZRLE", "frames, megabits and the encoding name are shown");
    assert_eq!(stats_line(0.0, 0, -9999), "0 к/с · 0.0 Мбит/с · -9999", "an unknown encoding is shown by its number");
}
