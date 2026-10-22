use crate::Rect;

#[test]
fn test_a_union_covers_both_rects() {
    let union = Rect::new(10, 20, 5, 5).union(Rect::new(0, 30, 2, 10));
    assert_eq!(union, Rect::new(0, 20, 15, 20), "the union spans from the leftmost top corner to the farthest bottom corner");
}
