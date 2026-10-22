use rfb::Rect;

use crate::Framebuffer;
use crate::test_support::rgba_pixel;

#[test]
fn test_a_rect_outside_the_frame_changes_nothing() {
    let mut framebuffer = Framebuffer::new(4, 4);
    framebuffer.fill_rect(Rect::new(3, 3, 2, 2), rgba_pixel(255, 255, 255));
    framebuffer.put_pixels(Rect::new(2, 0, 3, 1), &[rgba_pixel(1, 1, 1); 3]);
    framebuffer.copy_rect(0, 0, Rect::new(3, 0, 2, 2));
    framebuffer.copy_rect(3, 3, Rect::new(0, 0, 2, 2));
    assert!(
        framebuffer
            .pixels()
            .iter()
            .all(|&pixel| pixel == rgba_pixel(0, 0, 0)),
        "writes that leave the frame are ignored whole"
    );
    assert!(
        framebuffer.read_rect(Rect::new(3, 3, 2, 2)).is_empty(),
        "a read that leaves the frame returns nothing"
    );
    assert_eq!(
        framebuffer.read_rect(Rect::new(0, 0, 0, 3)),
        Vec::<u32>::new(),
        "an empty rect reads as no pixels"
    );
}
