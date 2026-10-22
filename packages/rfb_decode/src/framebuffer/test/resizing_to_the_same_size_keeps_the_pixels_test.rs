use rfb::Rect;

use crate::Framebuffer;
use crate::test_support::rgba_pixel;

#[test]
fn test_resizing_to_the_same_size_keeps_the_pixels() {
    let mut framebuffer = Framebuffer::new(3, 2);
    framebuffer.fill_rect(Rect::new(0, 0, 3, 2), rgba_pixel(9, 8, 7));
    framebuffer.resize(3, 2);
    assert!(
        framebuffer
            .pixels()
            .iter()
            .all(|&pixel| pixel == rgba_pixel(9, 8, 7)),
        "an unchanged size must not blank the screen"
    );
}
