use rfb::Rect;
use rfb_decode::Framebuffer;

use super::merge_changed_rects;
use crate::Patch;

pub(crate) fn collect_patches(framebuffer: &Framebuffer, changed: &[Rect]) -> Vec<Patch> {
    merge_changed_rects(changed)
        .into_iter()
        .map(|rect| Patch {
            rect,
            pixels: framebuffer.read_rect(rect),
        })
        .collect()
}
