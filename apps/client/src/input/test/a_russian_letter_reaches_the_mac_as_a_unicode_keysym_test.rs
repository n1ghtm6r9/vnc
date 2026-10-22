use remote_input::KeyMapping;
use rfb_session::ClientCommand;
use winit::keyboard::{KeyCode, PhysicalKey};

use crate::InputRouter;

#[test]
fn test_a_russian_letter_reaches_the_mac_as_a_unicode_keysym() {
    let mut router = InputRouter::new(KeyMapping::default());
    let press = router.key(PhysicalKey::Code(KeyCode::KeyQ), Some('й'), true);
    assert!(
        matches!(press, Some(ClientCommand::Key { keysym: 0x0100_0439, down: true })),
        "Q in the Russian Windows layout types й on the Mac, got {press:?}"
    );
}
