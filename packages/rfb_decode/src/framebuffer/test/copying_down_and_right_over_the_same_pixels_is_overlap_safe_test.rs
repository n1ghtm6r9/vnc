use rfb::Rect;

use crate::Framebuffer;
use crate::test_support::rgba_pixel;

#[test]
fn test_copying_down_and_right_over_the_same_pixels_is_overlap_safe() {
    let mut framebuffer = Framebuffer::new(6, 6);
    let numbered: Vec<u32> = (0..36).map(|index| rgba_pixel(index, 0, 0)).collect();
    framebuffer.put_pixels(Rect::new(0, 0, 6, 6), &numbered);
    framebuffer.copy_rect(0, 0, Rect::new(1, 1, 4, 4));
    for y in 1..5usize {
        for x in 1..5usize {
            assert_eq!(
                framebuffer.pixels()[y * 6 + x],
                numbered[(y - 1) * 6 + (x - 1)],
                "pixel ({x}, {y}) holds the original pixel one step up and left"
            );
        }
    }
    assert_eq!(
        framebuffer.pixels()[0],
        numbered[0],
        "the pixels outside the destination are untouched"
    );
}
