use crate::UNICODE_KEYSYM_BASE;

pub fn keysym_for_character(character: char) -> Option<u32> {
    if character.is_control() {
        return None;
    }
    let code_point = u32::from(character);
    if code_point <= 0xff {
        return Some(code_point);
    }
    Some(UNICODE_KEYSYM_BASE | code_point)
}
