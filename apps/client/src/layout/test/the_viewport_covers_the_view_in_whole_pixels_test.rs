use gpu_scaler::Viewport;
use remote_input::ViewRect;

use crate::viewport_for_view;

#[test]
fn test_the_viewport_covers_the_view_in_whole_pixels() {
    let fitted = ViewRect { x: 215.0, y: 0.0, width: 2130.0, height: 1377.0 };
    assert_eq!(
        viewport_for_view(fitted),
        Viewport { x: 215, y: 0, width: 2130, height: 1377 },
        "a fitted view becomes the same viewport"
    );
    let panned = ViewRect { x: -448.0, y: -397.0, width: 3456.0, height: 2234.0 };
    assert_eq!(
        viewport_for_view(panned),
        Viewport { x: -448, y: -397, width: 3456, height: 2234 },
        "a panned 1:1 view keeps its negative origin"
    );
}
