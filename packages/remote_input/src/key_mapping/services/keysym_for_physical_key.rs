use winit::keyboard::{KeyCode, PhysicalKey};

use super::swap_control_and_command_code;
use crate::{CommandKeysym, KEYSYM_META_LEFT, KEYSYM_META_RIGHT, KeyMapping, keysym_for_key_code};

pub fn keysym_for_physical_key(key: PhysicalKey, mapping: KeyMapping) -> Option<u32> {
    let PhysicalKey::Code(code) = key else {
        return None;
    };
    let code = if mapping.swap_control_and_command {
        swap_control_and_command_code(code)
    } else {
        code
    };
    match (code, mapping.command_keysym) {
        (KeyCode::SuperLeft, CommandKeysym::Meta) => Some(KEYSYM_META_LEFT),
        (KeyCode::SuperRight, CommandKeysym::Meta) => Some(KEYSYM_META_RIGHT),
        _ => keysym_for_key_code(code),
    }
}
