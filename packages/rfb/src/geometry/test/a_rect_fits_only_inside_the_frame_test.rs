use crate::Rect;

#[test]
fn test_a_rect_fits_only_inside_the_frame() {
    assert!(Rect::new(0, 0, 3456, 2234).fits_within(3456, 2234), "a rect the size of the frame fits it");
    assert!(!Rect::new(1, 0, 3456, 2234).fits_within(3456, 2234), "one pixel past the right edge does not fit");
    assert!(!Rect::new(0, 65535, 1, 2).fits_within(65535, 65535), "coordinates near u16::MAX must not wrap into a fit");
}
