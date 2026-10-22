use crate::{AppleDisplayGeometry, apple_display_geometry};

#[test]
fn test_the_apple_display_geometry_holds_points_and_pixels() {
    let header = [
        0x00, 0x05, 0x05, 0xe8, 0x03, 0xd6, 0x0b, 0xd0, 0x07, 0xac, 0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00,
        0x04, 0x00, 0x01,
    ];

    assert_eq!(
        apple_display_geometry(&header),
        Some(AppleDisplayGeometry {
            pixel_width: 3024,
            pixel_height: 1964,
            point_width: 1512,
            point_height: 982,
        }),
        "the display info 2 header of the MacBook describes 1512x982 points drawn with 3024x1964 pixels"
    );
}
