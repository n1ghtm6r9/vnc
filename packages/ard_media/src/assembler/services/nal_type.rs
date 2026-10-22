pub fn nal_type(first_byte: u8) -> u8 {
    (first_byte >> 1) & 0x3f
}
