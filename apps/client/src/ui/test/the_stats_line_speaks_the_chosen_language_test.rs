use rfb_session::SessionStats;

use crate::{Language, stats_line};

#[test]
fn test_the_stats_line_speaks_the_chosen_language() {
    let stats = SessionStats { bytes_per_second: 3_000_000, updates_per_second: 60.0, last_encoding: 1010 };
    for (language, expected) in [
        (Language::English, "60 fps · 24.0 Mbit/s · High Performance"),
        (Language::Russian, "60 к/с · 24.0 Мбит/с · High Performance"),
        (Language::Ukrainian, "60 к/с · 24.0 Мбіт/с · High Performance"),
    ] {
        assert_eq!(stats_line(stats, language), expected, "the units follow the {language:?} interface");
    }
}
