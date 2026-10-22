use rfb::Rect;

use crate::Framebuffer;
use crate::test_support::rgba_pixel;

#[test]
fn test_copying_along_one_row_is_overlap_safe() {
    let mut framebuffer = Framebuffer::new(8, 2);
    let numbered: Vec<u32> = (0..16).map(|index| rgba_pixel(index, 0, 0)).collect();
    framebuffer.put_pixels(Rect::new(0, 0, 8, 2), &numbered);
    framebuffer.copy_rect(0, 0, Rect::new(2, 0, 5, 1));
    assert_eq!(
        framebuffer.read_rect(Rect::new(0, 0, 8, 1)),
        vec![
            numbered[0],
            numbered[1],
            numbered[0],
            numbered[1],
            numbered[2],
            numbered[3],
            numbered[4],
            numbered[7]
        ],
        "a row shifted right over itself keeps the original order"
    );
    assert_eq!(
        framebuffer.read_rect(Rect::new(0, 1, 8, 1)),
        numbered[8..16].to_vec(),
        "the next row is untouched"
    );
}
