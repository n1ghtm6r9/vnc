use crate::{FRAMES_PER_SECOND_UNIT, MEGABITS_PER_SECOND_UNIT, encoding_name};

pub fn stats_line(updates_per_second: f32, bytes_per_second: u64, encoding: i32) -> String {
    let megabits = bytes_per_second as f64 * 8.0 / 1_000_000.0;
    format!(
        "{updates_per_second:.0} {FRAMES_PER_SECOND_UNIT} · {megabits:.1} {MEGABITS_PER_SECOND_UNIT} · {}",
        encoding_name(encoding)
    )
}
