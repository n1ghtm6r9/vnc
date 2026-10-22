use crate::{StreamFormat, SurfacePlane, test_support::test_video_format};

#[test]
fn test_a_444_surface_copies_three_whole_planes() {
    let format = StreamFormat::read(&test_video_format()).expect("the test format is supported");

    let planes = format.planes();

    let plane = |index| SurfacePlane {
        index,
        x_bytes: 0,
        y: 0,
        width_bytes: 200,
        rows: 150,
    };
    assert_eq!(
        planes,
        [plane(0), plane(1), plane(2)],
        "luma and both chroma planes are copied whole from the display area"
    );
}
