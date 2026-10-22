pub fn apple_display_info_payload(pixel_width: u16, pixel_height: u16) -> Vec<u8> {
    let mut payload = vec![0, 20, 0, 1, 0, 0, 0, 0];
    payload.extend_from_slice(&pixel_width.to_be_bytes());
    payload.extend_from_slice(&pixel_height.to_be_bytes());
    payload.extend_from_slice(&[0; 10]);
    payload
}
