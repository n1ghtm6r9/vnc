use rfb::Rect;

use crate::Framebuffer;
use crate::test_support::rgba_pixel;

#[test]
fn test_put_pixels_and_read_rect_round_trip() {
    let mut framebuffer = Framebuffer::new(6, 4);
    let block: Vec<u32> = (0..6).map(|index| rgba_pixel(index * 10, 1, 2)).collect();
    framebuffer.put_pixels(Rect::new(2, 1, 3, 2), &block);
    assert_eq!(
        framebuffer.read_rect(Rect::new(2, 1, 3, 2)),
        block,
        "the block reads back as it was put"
    );
    assert_eq!(
        framebuffer.pixels()[6 + 1],
        rgba_pixel(0, 0, 0),
        "the pixel left of the block is untouched"
    );
    assert_eq!(
        framebuffer.pixels()[6 + 5],
        rgba_pixel(0, 0, 0),
        "the pixel right of the block is untouched"
    );
    assert_eq!(
        framebuffer.pixels()[2 * 6 + 2],
        rgba_pixel(30, 1, 2),
        "the second block row starts at the next frame row"
    );
    assert_eq!(
        framebuffer.read_rect(Rect::new(3, 2, 2, 1)),
        vec![rgba_pixel(40, 1, 2), rgba_pixel(50, 1, 2)],
        "a read of part of the block returns only that part"
    );
}
