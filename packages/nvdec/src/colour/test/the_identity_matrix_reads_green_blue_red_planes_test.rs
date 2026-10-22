use crate::{ColourMatrix, ColourPrimaries, ColourTransform};

#[test]
fn test_the_identity_matrix_reads_green_blue_red_planes() {
    let full = ColourTransform::new(ColourMatrix::Identity, true, ColourPrimaries::Bt709);
    let limited = ColourTransform::new(ColourMatrix::Identity, false, ColourPrimaries::Bt709);

    assert_eq!(
        full.encoded_rgb(10, 20, 30),
        [30, 10, 20],
        "full range identity takes green from luma, blue from the first chroma plane and red from the second"
    );
    assert_eq!(
        limited.encoded_rgb(16, 16, 16),
        [0; 3],
        "limited identity black is 16 on every plane"
    );
    assert_eq!(
        limited.encoded_rgb(235, 235, 235),
        [255; 3],
        "limited identity white is 235 on every plane"
    );
}
