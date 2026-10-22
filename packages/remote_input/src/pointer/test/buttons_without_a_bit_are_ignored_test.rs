use winit::event::MouseButton;

use crate::button_bit;

#[test]
fn test_buttons_without_a_bit_are_ignored() {
    assert_eq!(button_bit(MouseButton::Forward), None, "the forward button has no bit in the mask");
    assert_eq!(button_bit(MouseButton::Other(7)), None, "an unnamed button has no bit in the mask");
}
