use winit::keyboard::PhysicalKey;

use crate::{KeyMapping, is_character_key, keysym_for_character, keysym_for_physical_key};

pub fn keysym_for_typed_key(
    key: PhysicalKey,
    character: Option<char>,
    mapping: KeyMapping,
    shortcut_held: bool,
) -> Option<u32> {
    let typed = match key {
        PhysicalKey::Code(code) if !shortcut_held && is_character_key(code) => {
            character.and_then(keysym_for_character)
        }
        _ => None,
    };
    typed.or_else(|| keysym_for_physical_key(key, mapping))
}
