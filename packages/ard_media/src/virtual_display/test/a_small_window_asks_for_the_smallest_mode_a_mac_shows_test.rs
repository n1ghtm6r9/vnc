use crate::virtual_display_for;

#[test]
fn test_a_small_window_asks_for_the_smallest_mode_a_mac_shows() {
    let cases = [
        (
            (1478, 480, 2),
            [3324, 1080, 1662, 540],
            "a short HiDPI display grows to 540 points high",
        ),
        (
            (1478, 480, 1),
            [1478, 480, 1478, 480],
            "at one pixel to the point 480 is enough",
        ),
        (
            (320, 240, 2),
            [1600, 1200, 800, 600],
            "a tiny HiDPI display grows to 800 points wide",
        ),
        (
            (320, 240, 1),
            [800, 600, 800, 600],
            "a tiny display grows to 800 wide",
        ),
        (
            (0, 0, 0),
            [800, 800, 800, 800],
            "a size of nothing becomes the smallest square",
        ),
    ];
    for ((width, height, scale), expected, meaning) in cases {
        let mode = virtual_display_for(width, height, scale).modes[0];
        assert_eq!(
            [mode.pixel_width, mode.pixel_height, mode.width, mode.height],
            expected,
            "{meaning}"
        );
    }
}
