use rfb::Rect;

pub fn rre_subrect(colour: [u8; 4], area: Rect) -> Vec<u8> {
    let mut bytes = colour.to_vec();
    for value in [area.x, area.y, area.width, area.height] {
        bytes.extend_from_slice(&value.to_be_bytes());
    }
    bytes
}
