use crate::{
    ColourMatrix, ColourPrimaries, ColourTransform, CuvidVideoFormat, StreamFormat,
    test_support::test_video_format,
};

#[test]
fn test_a_new_picture_size_needs_new_surfaces() {
    let current = StreamFormat::read(&test_video_format()).expect("the test format is supported");
    let recoloured = StreamFormat {
        colour: ColourTransform::new(ColourMatrix::Bt601, true, ColourPrimaries::DisplayP3),
        ..current
    };
    let taller = StreamFormat::read(&CuvidVideoFormat {
        coded_height: 176,
        ..test_video_format()
    })
    .expect("a taller test format is supported");

    assert!(
        current.same_surfaces(&recoloured),
        "a new colour description keeps the decoder"
    );
    assert!(
        !current.same_surfaces(&taller),
        "a new coded size needs a new decoder"
    );
}
