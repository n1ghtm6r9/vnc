use remote_input::KeyMapping;
use rfb_session::ClientCommand;
use winit::event::MouseScrollDelta;

use crate::InputRouter;

#[test]
fn test_a_wheel_in_apple_mode_scrolls_by_pixels() {
    let mut router = InputRouter::new(KeyMapping::default());
    router.set_apple_scroll(true);

    let commands = router.wheel(MouseScrollDelta::LineDelta(0.0, -1.0));
    let nothing = router.wheel(MouseScrollDelta::LineDelta(0.0, 0.0));

    assert_eq!(
        commands,
        vec![ClientCommand::Scroll { x: 0, y: 0, right: 0, down: 100 }],
        "in Apple Remote Desktop mode a notch becomes one smooth scroll instead of wheel button clicks"
    );
    assert!(nothing.is_empty(), "an empty wheel event sends nothing, got {nothing:?}");
}
