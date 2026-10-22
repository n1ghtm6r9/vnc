use crate::{AppleScroll, write_apple_scroll};

#[test]
fn test_a_small_apple_scroll_still_moves_one_line() {
    let mut bytes = Vec::new();
    write_apple_scroll(&mut bytes, AppleScroll { x: 0, y: 0, right: -10, down: 10 }).unwrap();
    let whole_lines = [
        i16::from_be_bytes([bytes[44], bytes[45]]),
        i16::from_be_bytes([bytes[46], bytes[47]]),
    ];
    assert_eq!(whole_lines, [-1, 1], "ten pixels round to no line, yet a scroll moves at least one line its way");
}
