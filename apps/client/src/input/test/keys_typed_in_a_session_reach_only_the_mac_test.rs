use crate::{LinkState, overlay_hears_keys};

#[test]
fn test_keys_typed_in_a_session_reach_only_the_mac() {
    assert!(
        !overlay_hears_keys(LinkState::Online),
        "in a session Tab, Space and every other key go to the Mac and never to the toolbar"
    );
}
