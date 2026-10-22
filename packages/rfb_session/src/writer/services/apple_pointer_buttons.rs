pub(crate) fn apple_pointer_buttons(buttons: u8) -> u8 {
    let middle = buttons & 0b010;
    let right = buttons & 0b100;
    (buttons & !0b110) | (middle << 1) | (right >> 1)
}
