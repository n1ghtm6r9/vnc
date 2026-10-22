use crate::APPLE_DISPLAY_INFO_HEADER_LENGTH;

pub fn apple_display_pixel_size(payload: &[u8]) -> Option<(u16, u16)> {
    if payload.len() < APPLE_DISPLAY_INFO_HEADER_LENGTH {
        return None;
    }
    let width = u16::from_be_bytes([payload[6], payload[7]]);
    let height = u16::from_be_bytes([payload[8], payload[9]]);
    Some((width, height))
}
