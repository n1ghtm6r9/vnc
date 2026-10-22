pub(crate) fn range_limit(value: i32) -> u8 {
    let wrapped = value & 0x3FF;
    match wrapped {
        0..128 => (wrapped + 128) as u8,
        128..512 => 255,
        512..896 => 0,
        _ => (wrapped - 896) as u8,
    }
}
