pub(crate) fn step_away_from_zero(value: i8, step: u32) -> i8 {
    let value = i32::from(value);
    let step = step as i32;
    (if value >= 0 {
        value + step
    } else {
        value - step
    }) as i8
}
