use rfb::Rect;

use crate::Framebuffer;
use crate::test_support::rgba_pixel;

#[test]
fn test_resize_keeps_the_overlap_and_blackens_the_new_area() {
    let mut framebuffer = Framebuffer::new(4, 3);
    let numbered: Vec<u32> = (0..12).map(|index| rgba_pixel(index + 1, 0, 0)).collect();
    framebuffer.put_pixels(Rect::new(0, 0, 4, 3), &numbered);

    framebuffer.resize(6, 2);
    assert_eq!(
        (framebuffer.width(), framebuffer.height()),
        (6, 2),
        "the new size is reported"
    );
    assert_eq!(
        framebuffer.pixels().len(),
        12,
        "the pixel store follows the new size"
    );
    assert_eq!(
        framebuffer.read_rect(Rect::new(0, 0, 4, 2)),
        numbered[0..8].to_vec(),
        "the overlapping area keeps its pixels"
    );
    assert_eq!(
        framebuffer.read_rect(Rect::new(4, 0, 2, 2)),
        vec![rgba_pixel(0, 0, 0); 4],
        "the new columns are opaque black"
    );

    framebuffer.resize(2, 2);
    assert_eq!(
        framebuffer.read_rect(Rect::new(0, 0, 2, 2)),
        vec![numbered[0], numbered[1], numbered[4], numbered[5]],
        "shrinking keeps the top left corner"
    );
}
