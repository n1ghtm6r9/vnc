use rfb::Rect;
use rfb_decode::Framebuffer;

use crate::{Patch, collect_patches};

#[test]
fn test_more_than_sixty_four_rects_become_one_bounding_patch() {
    let mut framebuffer = Framebuffer::new(200, 10);
    framebuffer.fill_rect(Rect::new(0, 0, 200, 10), 0xFF33_6699);
    framebuffer.fill_rect(Rect::new(100, 0, 100, 10), 0xFF99_6633);
    let changed: Vec<Rect> = (0..65u16)
        .map(|index| Rect::new(index * 3, index % 10, 1, 1))
        .collect();

    let patches = collect_patches(&framebuffer, &changed);

    let bounds = Rect::new(0, 0, 193, 10);
    assert_eq!(
        patches,
        vec![Patch {
            rect: bounds,
            pixels: framebuffer.read_rect(bounds),
        }],
        "65 scattered rects arrive as one patch over their bounds"
    );
}
