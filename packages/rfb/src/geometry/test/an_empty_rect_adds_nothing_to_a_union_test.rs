use crate::Rect;

#[test]
fn test_an_empty_rect_adds_nothing_to_a_union() {
    let rect = Rect::new(5, 5, 10, 10);
    assert_eq!(Rect::default().union(rect), rect, "an empty rect at the origin must not stretch the union to the origin");
    assert_eq!(rect.union(Rect::new(100, 100, 0, 7)), rect, "a zero-width rect is empty wherever it is");
}
