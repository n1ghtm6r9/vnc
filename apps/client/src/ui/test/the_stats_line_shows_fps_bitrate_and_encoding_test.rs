use rfb_session::SessionStats;

use crate::{Language, stats_line};

#[test]
fn test_the_stats_line_shows_fps_bitrate_and_encoding() {
    let busy = SessionStats { bytes_per_second: 2_500_000, updates_per_second: 59.6, last_encoding: 16 };
    let unknown = SessionStats { bytes_per_second: 0, updates_per_second: 0.0, last_encoding: -9999 };
    assert_eq!(
        stats_line(busy, Language::English),
        "60 fps · 20.0 Mbit/s · ZRLE",
        "frames, megabits and the encoding name are shown"
    );
    assert_eq!(
        stats_line(unknown, Language::English),
        "0 fps · 0.0 Mbit/s · -9999",
        "an unknown encoding is shown by its number"
    );
}
