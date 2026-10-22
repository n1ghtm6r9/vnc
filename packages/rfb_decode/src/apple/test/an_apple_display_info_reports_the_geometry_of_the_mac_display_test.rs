use rfb::{AppleDisplayGeometry, ENCODING_APPLE_DISPLAY_INFO_2, Rect};

use crate::RectOutcome;
use crate::test_support::DecodeHarness;

#[test]
fn test_an_apple_display_info_reports_the_geometry_of_the_mac_display() {
    let mut harness = DecodeHarness::new(4, 4);
    let mut payload = vec![0, 20, 0, 5, 0, 3, 0, 2, 0, 6, 0, 4];
    payload.extend_from_slice(&[0; 10]);

    let outcome = harness
        .decode(
            ENCODING_APPLE_DISPLAY_INFO_2,
            Rect::new(0, 0, 0, 0),
            &payload,
        )
        .expect("a display info decodes");

    assert_eq!(
        outcome,
        RectOutcome::AppleDisplay {
            width: 6,
            height: 4,
            geometry: Some(AppleDisplayGeometry {
                pixel_width: 6,
                pixel_height: 4,
                point_width: 3,
                point_height: 2,
            }),
        },
        "the desktop takes the pixel size, and the size in points comes along with it"
    );
}
