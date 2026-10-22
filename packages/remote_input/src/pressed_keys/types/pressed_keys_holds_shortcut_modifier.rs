use super::PressedKeys;
use crate::SHORTCUT_MODIFIER_KEYSYMS;

impl PressedKeys {
    pub fn holds_shortcut_modifier(&self) -> bool {
        self.keys
            .iter()
            .any(|(_, keysym)| SHORTCUT_MODIFIER_KEYSYMS.contains(keysym))
    }
}
