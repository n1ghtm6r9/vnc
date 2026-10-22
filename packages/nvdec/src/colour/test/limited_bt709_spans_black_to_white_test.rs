use crate::{ColourMatrix, ColourPrimaries, ColourTransform};

#[test]
fn test_limited_bt709_spans_black_to_white() {
    let transform = ColourTransform::new(ColourMatrix::Bt709, false, ColourPrimaries::Bt709);

    assert_eq!(
        transform.encoded_rgb(235, 128, 128),
        [255; 3],
        "studio white becomes full white"
    );
    assert_eq!(
        transform.encoded_rgb(16, 128, 128),
        [0; 3],
        "studio black becomes full black"
    );
    assert_eq!(
        transform.encoded_rgb(0, 128, 128),
        [0; 3],
        "footroom stays black"
    );
    assert_eq!(
        transform.encoded_rgb(255, 128, 128),
        [255; 3],
        "headroom stays white"
    );
}
