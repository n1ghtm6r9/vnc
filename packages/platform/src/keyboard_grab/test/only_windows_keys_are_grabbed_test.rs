use crate::should_grab;

#[test]
fn test_only_windows_keys_are_grabbed() {
    assert!(should_grab(0x5b, true, true), "the left Windows key goes to the Mac as Command");
    assert!(should_grab(0x5c, true, true), "the right Windows key goes to the Mac as Command");
    for virtual_key in [0x41, 0x09, 0x12, 0x11, 0x10, 0x1b, 0x73] {
        assert!(!should_grab(virtual_key, true, true), "key {virtual_key:#x} reaches the window normally");
    }
}
