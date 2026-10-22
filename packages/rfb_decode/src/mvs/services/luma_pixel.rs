pub(crate) fn luma_pixel(luma: u8, offsets: [i32; 3]) -> u32 {
    let [red, green, blue] = offsets.map(|offset| (i32::from(luma) + offset).clamp(0, 255) as u8);
    u32::from_le_bytes([red, green, blue, 255])
}
