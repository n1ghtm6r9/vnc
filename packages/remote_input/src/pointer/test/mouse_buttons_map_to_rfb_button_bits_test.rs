use winit::event::MouseButton;

use crate::button_bit;

#[test]
fn test_mouse_buttons_map_to_rfb_button_bits() {
    assert_eq!(button_bit(MouseButton::Left), Some(1), "the left button is bit 0 of the RFB button mask");
    assert_eq!(button_bit(MouseButton::Middle), Some(2), "the middle button is bit 1");
    assert_eq!(button_bit(MouseButton::Right), Some(4), "the right button is bit 2");
    assert_eq!(button_bit(MouseButton::Back), Some(128), "the back button is bit 7");
}
