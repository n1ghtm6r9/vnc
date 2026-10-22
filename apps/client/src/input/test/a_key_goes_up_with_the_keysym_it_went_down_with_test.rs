use remote_input::KeyMapping;
use rfb_session::ClientCommand;
use winit::keyboard::{KeyCode, PhysicalKey};

use crate::InputRouter;

#[test]
fn test_a_key_goes_up_with_the_keysym_it_went_down_with() {
    let mut router = InputRouter::new(KeyMapping::default());
    let shift = PhysicalKey::Code(KeyCode::ShiftLeft);
    let q = PhysicalKey::Code(KeyCode::KeyQ);
    router.key(shift, None, true);
    router.key(q, Some('Й'), true);
    router.key(shift, None, false);
    let release = router.key(q, Some('й'), false);
    assert!(
        matches!(release, Some(ClientCommand::Key { keysym: 0x0100_0419, down: false })),
        "Й typed with Shift goes up as Й after Shift is let go, so no key stays held on the Mac, got {release:?}"
    );
}
