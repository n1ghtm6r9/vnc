use crate::Viewport;

#[test]
fn test_a_viewport_past_the_target_clips_to_nothing() {
    assert_eq!(
        Viewport::new(100, 0, 10, 10).visible_within(100, 100),
        None,
        "a viewport that starts at the right edge shows nothing"
    );
    assert_eq!(
        Viewport::new(0, 250, 10, 10).visible_within(100, 100),
        None,
        "a viewport far below the target shows nothing"
    );
    assert_eq!(
        Viewport::new(-10, 0, 10, 10).visible_within(100, 100),
        None,
        "a viewport that ends at the left edge shows nothing"
    );
    assert_eq!(
        Viewport::new(0, -10, 10, 10).visible_within(100, 100),
        None,
        "a viewport that ends at the top edge shows nothing"
    );
    assert_eq!(
        Viewport::new(i32::MAX, i32::MAX, 5, 5).visible_within(100, 100),
        None,
        "an origin near i32::MAX must not wrap into the target"
    );
    assert_eq!(
        Viewport::new(i32::MIN, 0, 5, 5).visible_within(100, 100),
        None,
        "an origin near i32::MIN must not wrap into the target"
    );
    assert_eq!(
        Viewport::new(5, 5, 0, 10).visible_within(100, 100),
        None,
        "no width means nothing to draw"
    );
    assert_eq!(
        Viewport::new(5, 5, 10, 0).visible_within(100, 100),
        None,
        "no height means nothing to draw"
    );
}
