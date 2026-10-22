use rfb::Rect;

use crate::Framebuffer;

pub(crate) fn write_rows(
    framebuffer: &mut Framebuffer,
    rect: Rect,
    data: &[u8],
    convert: impl Fn(&[u8], &mut [u32]),
) {
    let Some(row_bytes) = data.len().checked_div(usize::from(rect.height)) else {
        return;
    };
    if row_bytes == 0 {
        return;
    }
    for (row, source) in (0..rect.height).zip(data.chunks_exact(row_bytes)) {
        convert(
            source,
            framebuffer.row_mut(rect.x, rect.y.saturating_add(row), rect.width),
        );
    }
}
