use rfb::Rect;

use crate::Framebuffer;
use crate::test_support::rgba_pixel;

#[test]
fn test_fill_rect_paints_only_the_rect() {
    let mut framebuffer = Framebuffer::new(6, 4);
    framebuffer.fill_rect(Rect::new(1, 1, 3, 2), rgba_pixel(200, 0, 0));
    for y in 0..4usize {
        for x in 0..6usize {
            let inside = (1..4).contains(&x) && (1..3).contains(&y);
            let expected = if inside {
                rgba_pixel(200, 0, 0)
            } else {
                rgba_pixel(0, 0, 0)
            };
            assert_eq!(
                framebuffer.pixels()[y * 6 + x],
                expected,
                "pixel ({x}, {y}) is painted only when it lies inside the rect"
            );
        }
    }
}
