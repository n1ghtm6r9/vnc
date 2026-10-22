use crate::should_grab;

#[test]
fn test_nothing_is_grabbed_when_inactive_or_unfocused() {
    assert!(!should_grab(0x5b, false, true), "an inactive grab leaves the Windows key to Windows");
    assert!(!should_grab(0x5b, true, false), "another foreground window keeps its Windows key");
    assert!(!should_grab(0x5c, false, false), "nothing is grabbed when inactive and unfocused");
}
