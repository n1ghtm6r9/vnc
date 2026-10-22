use crate::{
    CHROMA_FORMAT_420, CuvidVideoFormat, StreamFormat, SurfacePlane,
    test_support::test_video_format,
};

#[test]
fn test_a_cropped_nv12_surface_halves_its_chroma_offsets() {
    let video = CuvidVideoFormat {
        chroma_format: CHROMA_FORMAT_420,
        display_left: 2,
        display_top: 4,
        display_right: 8,
        display_bottom: 9,
        ..test_video_format()
    };

    let planes = StreamFormat::read(&video)
        .expect("a cropped 4:2:0 stream is supported")
        .planes();

    assert_eq!(
        planes,
        [
            SurfacePlane {
                index: 0,
                x_bytes: 2,
                y: 4,
                width_bytes: 6,
                rows: 5
            },
            SurfacePlane {
                index: 1,
                x_bytes: 2,
                y: 2,
                width_bytes: 6,
                rows: 3
            },
        ],
        "the interleaved chroma plane starts at half the luma rows and covers the rounded up half height"
    );
}
