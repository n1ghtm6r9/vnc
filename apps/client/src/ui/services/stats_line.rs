use rfb_session::SessionStats;

use crate::{FRAMES_PER_SECOND_UNIT, Language, MEGABITS_PER_SECOND_UNIT, encoding_name};

pub fn stats_line(stats: SessionStats, language: Language) -> String {
    let megabits = stats.bytes_per_second as f64 * 8.0 / 1_000_000.0;
    let (frames_unit, megabits_unit) = (FRAMES_PER_SECOND_UNIT.text(language), MEGABITS_PER_SECOND_UNIT.text(language));
    format!(
        "{:.0} {frames_unit} · {megabits:.1} {megabits_unit} · {}",
        stats.updates_per_second,
        encoding_name(stats.last_encoding)
    )
}
