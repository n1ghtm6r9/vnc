use rfb_session::SessionStats;

use crate::{Language, stats_line};

#[test]
fn test_the_stats_line_names_the_apple_mvs_encoding() {
    let stats = SessionStats { bytes_per_second: 1_000_000, updates_per_second: 60.0, last_encoding: 1011 };
    assert_eq!(
        stats_line(stats, Language::English),
        "60 fps · 8.0 Mbit/s · Apple MVS",
        "the Mac's own encoding is shown by name"
    );
}
