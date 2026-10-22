use remote_input::KeyMapping;
use rfb_session::ClientCommand;
use winit::event::{MouseButton, MouseScrollDelta};

use crate::InputRouter;

#[test]
fn test_each_wheel_step_presses_and_releases_its_button() {
    let mut router = InputRouter::new(KeyMapping::default());
    router.button(MouseButton::Left, true);
    let commands = router.wheel(MouseScrollDelta::LineDelta(0.0, 2.0));
    let buttons: Vec<u8> = commands
        .iter()
        .filter_map(|command| match command {
            ClientCommand::Pointer { buttons, .. } => Some(*buttons),
            _ => None,
        })
        .collect();
    assert_eq!(buttons, vec![9, 1, 9, 1], "two lines up press and release button 4 twice while the left button stays held");
}
