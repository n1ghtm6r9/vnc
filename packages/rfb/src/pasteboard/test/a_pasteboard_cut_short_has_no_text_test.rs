use crate::{apple_pasteboard, apple_pasteboard_text};

#[test]
fn test_a_pasteboard_cut_short_has_no_text() {
    let whole = apple_pasteboard("hello");
    for length in 0..whole.len() {
        assert_eq!(apple_pasteboard_text(&whole[..length]), None, "a pasteboard cut to {length} bytes has no text");
    }
    assert_eq!(apple_pasteboard_text(&whole), Some("hello".to_owned()), "the whole pasteboard has its text");
}
