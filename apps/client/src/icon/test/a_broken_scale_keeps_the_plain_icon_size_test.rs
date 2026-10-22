use crate::{BIG_ICON_SIDE, SMALL_ICON_SIDE, icon_side};

#[test]
fn test_a_broken_scale_keeps_the_plain_icon_size() {
    assert_eq!(icon_side(SMALL_ICON_SIDE, f64::NAN), 16, "an unknown scale keeps the title bar icon at 16 pixels");
    assert_eq!(icon_side(BIG_ICON_SIDE, 0.0), 32, "a zero scale keeps the taskbar icon at 32 pixels");
}
