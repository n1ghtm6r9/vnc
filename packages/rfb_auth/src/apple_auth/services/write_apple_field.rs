pub(crate) fn write_apple_field(field: &mut [u8], text: &str) {
    let kept = text.floor_char_boundary(field.len() - 1);
    field[..kept].copy_from_slice(&text.as_bytes()[..kept]);
    field[kept] = 0;
}
