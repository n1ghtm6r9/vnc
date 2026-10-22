use crate::{
    ColourMatrix, ColourPrimaries, ColourTransform,
    test_support::{BLUE, RED, assert_rgb_near},
};

#[test]
fn test_full_range_bt601_keeps_its_extremes() {
    let transform = ColourTransform::new(ColourMatrix::Bt601, true, ColourPrimaries::Bt709);

    assert_eq!(
        transform.encoded_rgb(255, 128, 128),
        [255; 3],
        "full white stays white"
    );
    assert_eq!(
        transform.encoded_rgb(0, 128, 128),
        [0; 3],
        "full black stays black"
    );
    assert_rgb_near(transform.encoded_rgb(76, 85, 255), RED, "full range red");
    assert_rgb_near(transform.encoded_rgb(29, 255, 107), BLUE, "full range blue");
}
