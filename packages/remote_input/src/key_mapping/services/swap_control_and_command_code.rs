use winit::keyboard::KeyCode;

pub(crate) fn swap_control_and_command_code(code: KeyCode) -> KeyCode {
    match code {
        KeyCode::ControlLeft => KeyCode::SuperLeft,
        KeyCode::SuperLeft => KeyCode::ControlLeft,
        KeyCode::ControlRight => KeyCode::SuperRight,
        KeyCode::SuperRight => KeyCode::ControlRight,
        other => other,
    }
}
