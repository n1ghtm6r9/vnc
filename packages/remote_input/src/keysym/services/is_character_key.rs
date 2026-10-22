use winit::keyboard::KeyCode;

use super::{digit_keysym, letter_keysym, punctuation_keysym};

pub(crate) fn is_character_key(code: KeyCode) -> bool {
    letter_keysym(code)
        .or_else(|| digit_keysym(code))
        .or_else(|| punctuation_keysym(code))
        .is_some()
}
