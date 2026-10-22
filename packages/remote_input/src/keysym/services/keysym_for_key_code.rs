use winit::keyboard::KeyCode;

use super::{
    digit_keysym, editing_keysym, function_keysym, keypad_keysym, letter_keysym, modifier_keysym,
    navigation_keysym, punctuation_keysym, system_keysym,
};

pub fn keysym_for_key_code(code: KeyCode) -> Option<u32> {
    letter_keysym(code)
        .or_else(|| digit_keysym(code))
        .or_else(|| punctuation_keysym(code))
        .or_else(|| editing_keysym(code))
        .or_else(|| navigation_keysym(code))
        .or_else(|| function_keysym(code))
        .or_else(|| modifier_keysym(code))
        .or_else(|| system_keysym(code))
        .or_else(|| keypad_keysym(code))
}
