use crate::virtual_display_for;

#[test]
fn test_a_display_size_keeps_within_the_pixels_of_4k() {
    let cases = [
        (
            (2560, 1440, 2),
            [3840, 2160, 1920, 1080],
            "a 5K picture shrinks to 4K in the same shape",
        ),
        (
            (5120, 2880, 1),
            [3840, 2160, 3840, 2160],
            "so does one at one pixel to the point",
        ),
        (
            (1200, 1600, 2),
            [2400, 3200, 1200, 1600],
            "a tall shape may be taller than 2160",
        ),
        (
            (2000, 600, 2),
            [4000, 1200, 2000, 600],
            "a wide shape may be wider than 3840",
        ),
        (
            (5000, 480, 2),
            [7680, 736, 3840, 368],
            "nothing is wider than 7680 pixels",
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
    assert_eq!(
        virtual_display_for(800, 5000, 1).modes[0].pixel_height,
        4320,
        "nothing is taller than 4320 pixels"
    );
}
