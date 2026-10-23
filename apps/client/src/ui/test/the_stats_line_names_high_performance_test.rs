use rfb_session::SessionStats;

use crate::{Language, stats_line};

#[test]
fn test_the_stats_line_names_high_performance() {
    let stats = SessionStats { bytes_per_second: 3_000_000, updates_per_second: 60.0, last_encoding: 1010 };
    assert_eq!(
        stats_line(stats, Language::English),
        "60 fps · 24.0 Mbit/s · High Performance",
        "the HEVC stream of the Mac is shown by name"
    );
}
