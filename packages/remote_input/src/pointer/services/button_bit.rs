use winit::event::MouseButton;

pub fn button_bit(button: MouseButton) -> Option<u8> {
    match button {
        MouseButton::Left => Some(1),
        MouseButton::Middle => Some(2),
        MouseButton::Right => Some(4),
        MouseButton::Back => Some(128),
        MouseButton::Forward | MouseButton::Other(_) => None,
    }
}
