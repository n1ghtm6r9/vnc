use remote_input::KeyMapping;
use rfb_session::ClientCommand;
use winit::keyboard::{KeyCode, PhysicalKey};

use crate::InputRouter;

#[test]
fn test_a_release_without_a_press_is_not_sent() {
    let mut router = InputRouter::new(KeyMapping::default());
    assert!(
        router.key(PhysicalKey::Code(KeyCode::KeyQ), None, false).is_none(),
        "a key pressed before the window had focus is not released on the Mac"
    );
    router.key(PhysicalKey::Code(KeyCode::KeyQ), None, true);
    let release = router.key(PhysicalKey::Code(KeyCode::KeyQ), None, false);
    assert!(
        matches!(release, Some(ClientCommand::Key { keysym: 0x71, down: false })),
        "a key pressed in the window is released on the Mac"
    );
}
