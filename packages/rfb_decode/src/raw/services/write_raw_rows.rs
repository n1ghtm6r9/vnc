use rfb::Rect;

use crate::{Framebuffer, PixelConverter, write_rows};

pub(crate) fn write_raw_rows(
    converter: &PixelConverter,
    framebuffer: &mut Framebuffer,
    rect: Rect,
    data: &[u8],
) {
    write_rows(framebuffer, rect, data, |source, target| {
        converter.rgba_row(source, target)
    });
}
