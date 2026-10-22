use remote_input::KeyMapping;
use rfb_session::ClientCommand;
use winit::event::MouseButton;
use winit::keyboard::{KeyCode, PhysicalKey};

use crate::InputRouter;

#[test]
fn test_focus_loss_releases_every_held_key() {
    let mut router = InputRouter::new(KeyMapping::default());
    router.key(PhysicalKey::Code(KeyCode::ShiftLeft), None, true);
    router.key(PhysicalKey::Code(KeyCode::KeyA), Some('a'), true);
    router.key(PhysicalKey::Code(KeyCode::KeyA), Some('A'), true);
    router.button(MouseButton::Left, true);
    let released = router.release_all();
    let mut keys: Vec<u32> = released
        .iter()
        .filter_map(|command| match command {
            ClientCommand::Key { keysym, down: false } => Some(*keysym),
            _ => None,
        })
        .collect();
    keys.sort_unstable();
    assert_eq!(keys, vec![0x61, 0xffe1], "shift and A are released once each even though A auto-repeated");
    assert!(
        matches!(released.last(), Some(ClientCommand::Pointer { buttons: 0, .. })),
        "the held mouse button is let go after the keys"
    );
    assert!(router.release_all().is_empty(), "a second focus loss has nothing left to release");
}
