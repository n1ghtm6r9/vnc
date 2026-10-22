use std::sync::Arc;

use crate::{GrabState, GrabbedKey};

#[test]
fn test_a_release_is_swallowed_only_when_its_press_was() {
    let mut state = GrabState {
        window: 1,
        active: true,
        on_key: Arc::new(|_| {}),
        swallowed: Vec::new(),
    };
    let press = GrabbedKey { virtual_key: 0x5b, down: true };
    let release = GrabbedKey { virtual_key: 0x5b, down: false };
    assert!(state.decide(press, true), "a press while connected and focused is swallowed");
    state.active = false;
    assert!(state.decide(press, false), "auto-repeat of a swallowed key stays swallowed after focus moves");
    assert!(state.decide(release, false), "the release of a swallowed press is swallowed too, so Windows never sees half a keystroke");
    assert!(!state.decide(release, true), "a second release has no press to pair with and passes on");
    assert!(!state.decide(press, true), "an inactive grab lets the press through");
    state.active = true;
    assert!(!state.decide(release, true), "the release of a press Windows saw also goes to Windows");
}
