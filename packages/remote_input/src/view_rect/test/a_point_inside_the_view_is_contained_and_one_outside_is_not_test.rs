use crate::ViewRect;

#[test]
fn test_a_point_inside_the_view_is_contained_and_one_outside_is_not() {
    let view = ViewRect { x: 200.0, y: 100.0, width: 1200.0, height: 900.0 };
    assert!(view.contains(700.0, 500.0), "the middle of the view is inside it");
    assert!(view.contains(200.0, 100.0), "the top-left corner belongs to the view");
    assert!(!view.contains(199.5, 500.0), "a point left of the view is outside it");
    assert!(!view.contains(1400.0, 500.0), "the right edge already belongs to the letterbox");
    assert!(!view.contains(700.0, 99.5), "a point above the view is outside it");
    assert!(!view.contains(700.0, 1000.0), "the bottom edge already belongs to the letterbox");
    assert!(!ViewRect::default().contains(0.0, 0.0), "an empty view contains nothing");
}
