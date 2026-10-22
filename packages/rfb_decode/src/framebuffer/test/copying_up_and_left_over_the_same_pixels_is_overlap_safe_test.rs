use rfb::Rect;

use crate::Framebuffer;
use crate::test_support::rgba_pixel;

#[test]
fn test_copying_up_and_left_over_the_same_pixels_is_overlap_safe() {
    let mut framebuffer = Framebuffer::new(6, 6);
    let numbered: Vec<u32> = (0..36).map(|index| rgba_pixel(index, 0, 0)).collect();
    framebuffer.put_pixels(Rect::new(0, 0, 6, 6), &numbered);
    framebuffer.copy_rect(1, 1, Rect::new(0, 0, 4, 4));
    for y in 0..4usize {
        for x in 0..4usize {
            assert_eq!(
                framebuffer.pixels()[y * 6 + x],
                numbered[(y + 1) * 6 + (x + 1)],
                "pixel ({x}, {y}) holds the original pixel one step down and right"
            );
        }
    }
}
