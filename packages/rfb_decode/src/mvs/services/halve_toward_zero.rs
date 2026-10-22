pub(crate) fn halve_toward_zero(value: i16) -> i32 {
    let value = i32::from(value);
    (value + i32::from(value < 0)) >> 1
}
