use crate::{VIRTUAL_DISPLAY_NAME, virtual_display_for};

#[test]
fn test_a_window_size_becomes_points_and_pixels() {
    let cases = [
        (
            (1920, 1080, 2),
            [3840, 2160, 1920, 1080],
            "a HiDPI display has twice the pixels",
        ),
        (
            (1920, 1080, 1),
            [1920, 1080, 1920, 1080],
            "a display at one pixel to the point",
        ),
        (
            (1365, 767, 1),
            [1366, 768, 1366, 768],
            "every dimension is made even",
        ),
        (
            (2558, 1377, 1),
            [2558, 1378, 2558, 1378],
            "a window of this machine fits as it is",
        ),
    ];
    for ((width, height, scale), expected, meaning) in cases {
        let display = virtual_display_for(width, height, scale);
        let mode = display.modes[0];
        assert_eq!(
            [mode.pixel_width, mode.pixel_height, mode.width, mode.height],
            expected,
            "{meaning}"
        );
        assert_eq!(mode.refresh, 60.0, "{meaning}: at 60 Hz");
        assert_eq!(
            display.name, VIRTUAL_DISPLAY_NAME,
            "{meaning}: under Apple's name"
        );
    }
}
