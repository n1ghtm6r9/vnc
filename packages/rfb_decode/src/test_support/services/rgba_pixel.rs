pub fn rgba_pixel(red: u8, green: u8, blue: u8) -> u32 {
    u32::from_le_bytes([red, green, blue, 255])
}
