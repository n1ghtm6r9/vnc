use crate::{Viewport, VisibleRegion};

#[test]
fn test_a_viewport_is_clipped_to_the_target() {
    assert_eq!(
        Viewport::new(10, 20, 30, 40).visible_within(100, 100),
        Some(VisibleRegion {
            x: 10,
            y: 20,
            width: 30,
            height: 40
        }),
        "a viewport inside the target is fully visible"
    );
    assert_eq!(
        Viewport::new(90, 80, 30, 40).visible_within(100, 100),
        Some(VisibleRegion {
            x: 90,
            y: 80,
            width: 10,
            height: 20
        }),
        "the part past the right and bottom edges is cut off"
    );
    assert_eq!(
        Viewport::new(-5, -3, 20, 10).visible_within(100, 100),
        Some(VisibleRegion {
            x: 0,
            y: 0,
            width: 15,
            height: 7
        }),
        "the part before the left and top edges is cut off"
    );
    assert_eq!(
        Viewport::new(-448, -397, 3456, 2234).visible_within(2560, 1440),
        Some(VisibleRegion {
            x: 0,
            y: 0,
            width: 2560,
            height: 1440
        }),
        "a panned one to one screen covers the whole window"
    );
    assert_eq!(
        Viewport::new(i32::MIN, i32::MIN, u32::MAX, u32::MAX).visible_within(64, 48),
        Some(VisibleRegion {
            x: 0,
            y: 0,
            width: 64,
            height: 48
        }),
        "an enormous viewport is cut to the target without overflow"
    );
}
