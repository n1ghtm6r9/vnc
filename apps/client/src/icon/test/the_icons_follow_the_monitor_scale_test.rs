use crate::{BIG_ICON_SIDE, SMALL_ICON_SIDE, icon_side};

#[test]
fn test_the_icons_follow_the_monitor_scale() {
    assert_eq!(icon_side(SMALL_ICON_SIDE, 1.0), 16, "at 100% the title bar takes a 16 pixel icon");
    assert_eq!(icon_side(SMALL_ICON_SIDE, 1.25), 20, "at 125% the title bar takes a 20 pixel icon");
    assert_eq!(icon_side(BIG_ICON_SIDE, 1.5), 48, "at 150% the taskbar takes a 48 pixel icon");
    assert_eq!(icon_side(BIG_ICON_SIDE, 2.0), 64, "at 200% the taskbar takes a 64 pixel icon");
}
