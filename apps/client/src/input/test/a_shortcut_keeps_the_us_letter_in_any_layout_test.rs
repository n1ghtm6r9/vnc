use remote_input::KeyMapping;
use rfb_session::ClientCommand;
use winit::keyboard::{KeyCode, PhysicalKey};

use crate::InputRouter;

#[test]
fn test_a_shortcut_keeps_the_us_letter_in_any_layout() {
    let mut router = InputRouter::new(KeyMapping::default());
    router.key(PhysicalKey::Code(KeyCode::SuperLeft), None, true);
    let press = router.key(PhysicalKey::Code(KeyCode::KeyC), Some('с'), true);
    assert!(
        matches!(press, Some(ClientCommand::Key { keysym: 0x63, down: true })),
        "Cmd+C copies on the Mac while Windows types Russian, got {press:?}"
    );
}
