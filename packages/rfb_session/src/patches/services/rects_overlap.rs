use rfb::Rect;

pub(crate) fn rects_overlap(first: Rect, second: Rect) -> bool {
    let first_right = u32::from(first.x) + u32::from(first.width);
    let first_bottom = u32::from(first.y) + u32::from(first.height);
    let second_right = u32::from(second.x) + u32::from(second.width);
    let second_bottom = u32::from(second.y) + u32::from(second.height);
    u32::from(first.x) < second_right
        && u32::from(second.x) < first_right
        && u32::from(first.y) < second_bottom
        && u32::from(second.y) < first_bottom
}
