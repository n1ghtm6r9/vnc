use crate::Framebuffer;
use crate::test_support::rgba_pixel;

#[test]
fn test_a_new_framebuffer_is_opaque_black() {
    let framebuffer = Framebuffer::new(5, 3);
    assert_eq!(framebuffer.width(), 5, "the width is kept");
    assert_eq!(framebuffer.height(), 3, "the height is kept");
    assert_eq!(
        framebuffer.pixels().len(),
        15,
        "there is one pixel per cell"
    );
    assert!(
        framebuffer
            .pixels()
            .iter()
            .all(|&pixel| pixel == rgba_pixel(0, 0, 0)),
        "every pixel starts opaque black"
    );
}
