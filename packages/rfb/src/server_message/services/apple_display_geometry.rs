use crate::{APPLE_DISPLAY_INFO_HEADER_LENGTH, AppleDisplayGeometry};

pub fn apple_display_geometry(payload: &[u8]) -> Option<AppleDisplayGeometry> {
    if payload.len() < APPLE_DISPLAY_INFO_HEADER_LENGTH {
        return None;
    }
    let sides = [2, 4, 6, 8].map(|offset| u16::from_be_bytes([payload[offset], payload[offset + 1]]));
    let [point_width, point_height, pixel_width, pixel_height] = sides;
    sides.iter().all(|&side| side > 0).then_some(AppleDisplayGeometry {
        pixel_width,
        pixel_height,
        point_width,
        point_height,
    })
}
