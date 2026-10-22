use crate::{CuvidVideoFormat, DisplayArea, test_support::test_video_format};

#[test]
fn test_a_display_area_outside_the_picture_falls_back_to_the_coded_size() {
    let inverted = CuvidVideoFormat {
        display_left: 50,
        display_right: 20,
        ..test_video_format()
    };
    let too_wide = CuvidVideoFormat {
        display_right: 500,
        ..test_video_format()
    };

    assert_eq!(
        DisplayArea::read(&inverted),
        DisplayArea {
            left: 0,
            top: 0,
            width: 208,
            height: 160
        },
        "an empty display area shows the whole coded picture"
    );
    assert_eq!(
        DisplayArea::read(&too_wide),
        DisplayArea {
            left: 0,
            top: 0,
            width: 208,
            height: 150
        },
        "a display area wider than the picture is clipped to it"
    );
}
