use crate::virtual_display_for;

#[test]
fn test_pixels_are_always_points_times_the_scale() {
    let sizes = [
        (1950, 480),
        (1976, 490),
        (3000, 300),
        (300, 3000),
        (641, 479),
        (5000, 400),
        (400, 5000),
        (1921, 1081),
        (974, 784),
    ];
    for scale in [1, 2] {
        for (width, height) in sizes {
            let mode = virtual_display_for(width, height, scale).modes[0];
            assert_eq!(
                [mode.pixel_width, mode.pixel_height],
                [mode.width * scale, mode.height * scale],
                "{width}x{height}@{scale}: the pixels are the points times the scale"
            );
            assert!(
                mode.pixel_width * mode.pixel_height <= 3840 * 2160
                    && mode.pixel_width <= 7680
                    && mode.pixel_height <= 4320,
                "{width}x{height}@{scale}: the picture stays within what the Mac encodes"
            );
        }
    }
}
