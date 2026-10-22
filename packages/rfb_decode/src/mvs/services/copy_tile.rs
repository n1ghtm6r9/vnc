use rfb::Rect;

use crate::Framebuffer;

pub(crate) fn copy_tile(framebuffer: &mut Framebuffer, source: (i32, i32), destination: Rect) {
    let (Ok(x), Ok(y)) = (u16::try_from(source.0), u16::try_from(source.1)) else {
        return;
    };
    let width = destination.width.min(framebuffer.width().saturating_sub(x));
    let height = destination
        .height
        .min(framebuffer.height().saturating_sub(y));
    framebuffer.copy_rect(x, y, Rect::new(destination.x, destination.y, width, height));
}
